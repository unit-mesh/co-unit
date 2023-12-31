use serde::{Deserialize, Serialize};
use crate::model::{CodeDataStruct, CodeFunction, ContainerDemand, ContainerService, ContainerSupply};
use crate::model::analyser::ApiAnalyser;

#[derive(Serialize, Deserialize, Clone)]
pub struct JavaApiAnalyser {
    resources: Vec<ContainerSupply>,
    demands: Vec<ContainerDemand>,
}

impl ApiAnalyser for JavaApiAnalyser {
    fn analysis_by_node(&mut self, node: &CodeDataStruct, workspace: String) {
        let route_annotation = node.filter_annotations(vec!["RestController", "Controller", "RequestMapping"]);

        // 1. create resources
        if !route_annotation.is_empty() {
            let mut base_url = String::new();
            let mapping_annotation = node.filter_annotations(vec!["RequestMapping"]);
            if !mapping_annotation.is_empty() && !mapping_annotation[0].key_values.is_empty() {
                let url = mapping_annotation[0].key_values[0].value.clone();
                base_url = url.trim_matches('"').to_string();
            }

            for func in &node.functions {
                self.create_resource(func.clone(), &base_url, node.clone());
            }
        }

        // 2. create demands
        let use_rest_template = node.imports.iter().any(|import| import.source.ends_with(".RestTemplate"));
        if use_rest_template {
            for func in &node.functions {
                let _ = self.create_demand(func.clone(), node.clone());
            }
        }
    }

    fn to_container_services(&self) -> Vec<ContainerService> {
        vec![ContainerService {
            name: "".to_string(),
            resources: self.resources.clone(),
            demands: self.demands.clone(),
        }]
    }

    fn new() -> Self {
        JavaApiAnalyser {
            resources: vec![],
            demands: vec![],
        }
    }
}

impl JavaApiAnalyser {
    fn create_demand(&mut self, func: CodeFunction, node: CodeDataStruct) -> anyhow::Result<(),  anyhow::Error> {
        for call in &func.function_calls {
            let mut function_name: String = call.function_name.clone();
            let node_name = call.node_name.clone();

            if function_name.contains('.') {
                function_name = function_name.split('.').last()
                    .ok_or(anyhow::anyhow!("split function name error"))?
                    .to_string();
            }

            if node_name == "RestTemplate" && node_name != "<init>" {
                let mut method = String::new();
                let lowercase = function_name.to_lowercase();
                match &*lowercase {
                    x if x.starts_with("get") => {
                        method = "Get".to_string();
                    }
                    x if x.starts_with("post") => {
                        method = "Post".to_string();
                    }
                    x if x.starts_with("delete") => {
                        method = "Delete".to_string();
                    }
                    x if x.starts_with("put") => {
                        method = "Put".to_string();
                    }
                    _ => {}
                }

                let mut url = String::new();
                if !call.parameters.is_empty() && !call.parameters[0].type_value.is_empty() {
                    url = call.parameters[0].type_value.clone().trim_matches('"').to_string();
                }

                if !method.is_empty() {
                    self.demands.push(ContainerDemand {
                        source_caller: node.node_name.clone(),
                        call_routes: vec![],
                        base: "".to_string(),
                        target_url: url.clone(),
                        target_http_method: method.clone(),
                        call_data: "".to_string(),
                    });
                }
            }
        }

        Ok(())
    }

    fn create_resource(&mut self, func: CodeFunction, base_url: &str, node: CodeDataStruct) {
        let mut http_method = String::new();
        let mut route = base_url.to_string();
        for annotation in &func.annotations {
            let mut is_http_annotation = true;
            match &annotation.name[..] {
                "GetMapping" => http_method = "Get".to_string(),
                "PostMapping" => http_method = "Post".to_string(),
                "DeleteMapping" => http_method = "Delete".to_string(),
                "PutMapping" => http_method = "Put".to_string(),
                "PatchMapping" => http_method = "Patch".to_string(),
                _ => is_http_annotation = false,
            }

            let has_sub_url_mapping = !annotation.key_values.is_empty();
            if is_http_annotation && !http_method.is_empty() && has_sub_url_mapping {
                let sub_url = annotation.key_values[0].value.clone();
                let pure_url = sub_url.trim_matches('"').to_string();

                if !base_url.is_empty() {
                    route = format!("{}{}", base_url, pure_url);
                } else {
                    route = pure_url;
                }
            }

            // todo: split by class
            // case 2
            if annotation.name == "RequestMapping" {
                let opt_url = annotation.key_values.iter().find(|kv| kv.key == "value");
                let opt_method = annotation.key_values.iter().find(|kv| kv.key == "method");
                if let (Some(url), Some(method)) = (opt_url, opt_method) {
                    match method.value.as_str() {
                        "RequestMethod.GET" | "GET" => http_method = "Get".to_string(),
                        "RequestMethod.POST" | "POST" => http_method = "Post".to_string(),
                        "RequestMethod.DELETE" | "DELETE" => http_method = "Delete".to_string(),
                        "RequestMethod.PUT" | "PUT" => http_method = "Put".to_string(),
                        "RequestMethod.PATCH" | "PATCH" => http_method = "Patch".to_string(),
                        _ => {}
                    }

                    let pure_url = url.value.trim_matches('"').to_string();
                    if !base_url.is_empty() {
                        route = format!("{}{}", base_url, pure_url);
                    } else {
                        route = pure_url;
                    }
                }
            }
        }

        if !http_method.is_empty() {
            if !route.starts_with('/') {
                route = format!("/{}", route);
            }

            route = route.replace("//", "/");

            self.resources.push(ContainerSupply {
                source_url: route.clone(),
                source_http_method: http_method.clone(),
                package_name: node.package.clone(),
                class_name: node.node_name.clone(),
                method_name: func.name.clone(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::analyser::api_analyser::JavaApiAnalyser;
    use crate::model::analyser::ApiAnalyser;
    use crate::model::CodeDataStruct;

    #[test]
    fn should_convert_api() {
        let demo_code = r#"[
  {
    "NodeName": "HelloController",
    "Type": "CLASS",
    "Package": "com.example.springboot",
    "FilePath": "HelloController.java",
    "Fields": [],
    "MultipleExtend": [],
    "Implements": [],
    "Functions": [
      {
        "Name": "index",
        "ReturnType": "String",
        "MultipleReturns": [],
        "Parameters": [],
        "FunctionCalls": [],
        "Annotations": [
          {
            "Name": "GetMapping",
            "KeyValues": [
              {
                "Key": "\"/\"",
                "Value": "\"/\""
              }
            ]
          }
        ],
        "Modifiers": [],
        "InnerStructures": [],
        "InnerFunctions": [],
        "Position": {
          "StartLine": 11,
          "StartLinePosition": 11,
          "StopLine": 13,
          "StopLinePosition": 4
        },
        "LocalVariables": []
      }
    ],
    "InnerStructures": [],
    "Annotations": [
      {
        "Name": "RestController",
        "KeyValues": []
      },
      {
        "Name": "RequestMapping",
        "KeyValues": []
      }
    ],
    "FunctionCalls": [],
    "Parameters": [],
    "Imports": [
      {
        "Source": "org.springframework.web.bind.annotation.GetMapping",
        "UsageName": []
      },
      {
        "Source": "org.springframework.web.bind.annotation.RestController",
        "UsageName": []
      }
    ],
    "Exports": [],
    "Position": {
      "StartLine": 8,
      "StartLinePosition": 7,
      "StopLine": 15
    }
  }
]"#;

        let mut converter = JavaApiAnalyser::new();
        let codes: Vec<CodeDataStruct> = serde_json::from_str(demo_code).unwrap();
        converter.analysis_by_node(&codes[0], "".to_string());
        let result = converter.to_container_services();
        assert_eq!(result.len(), 1);
        let supply = &result[0].resources[0];
        assert_eq!(supply.source_url, "/");
        assert_eq!(supply.source_http_method, "Get");
        assert_eq!(supply.package_name, "com.example.springboot");
        assert_eq!(supply.class_name, "HelloController");
        assert_eq!(supply.method_name, "index");
    }
}