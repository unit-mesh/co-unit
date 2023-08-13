use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CodeDataStruct {
    // class and DataStruct Name
    // for TypeScript/JavaScript, if it is a variable, function, it will be named `default`
    pub(crate) node_name: String,
    #[serde(default = "String::new")]
    pub(crate) module: String,
    #[serde(default = "DataStructType::default")]
    pub(crate) data_type: DataStructType, // You need to define DataStructType enum separately
    #[serde(default = "String::new")]
    pub(crate) package: String,
    #[serde(default = "String::new")]
    pub(crate) file_path: String,
    // todo: thinking of changing to property
    #[serde(default)]
    pub(crate) fields: Vec<CodeField>,
    #[serde(default)]
    pub(crate) multiple_extend: Vec<String>,
    #[serde(default)]
    pub(crate) implements: Vec<String>,
    #[serde(default)]
    pub(crate) extend: String,
    #[serde(default)]
    pub(crate) functions: Vec<CodeFunction>,
    #[serde(default)]
    pub(crate) inner_structures: Vec<CodeDataStruct>,
    #[serde(default)]
    pub(crate) annotations: Vec<CodeAnnotation>,
    #[serde(default)]
    pub(crate) function_calls: Vec<CodeCall>,
    #[deprecated(note = "looking for constructor method for SCALA")]
    #[serde(default)]
    pub(crate) parameters: Vec<CodeProperty>, // for Scala
    #[serde(default)]
    pub(crate) imports: Vec<CodeImport>,
    // in TypeScript, a file can export Function, Variable, Class, Interface
    // `export const baseURL = '/api'`
    #[serde(default)]
    pub(crate) exports: Vec<CodeExport>,
    // todo: select node use only imports
    pub(crate) extension: Option<JsonElement>, // You need to define JsonElement type separately
    pub(crate) position: CodePosition,
}

impl CodeDataStruct {
    pub(crate) fn filter_annotations(&self, keys: Vec<&str>) -> Vec<CodeAnnotation> {
        self.annotations
            .iter()
            .filter(|prop| keys.iter().any(|key| key.to_lowercase() == prop.name.to_lowercase()))
            .cloned()
            .collect()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum DataStructType {
    Empty,
    Default,
    Class,
    Variable,
    Interface,
    Struct,
    Object,
    InnerStructures,
    CreatorClass,
    AbstractClass,
    Trait,
    Enum,
}
impl DataStructType {
    pub fn default() -> Self {
        DataStructType::Default
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CodeField {
    pub(crate) type_type: Option<String>,
    pub(crate) type_value: Option<String>,
    pub(crate) type_key: Option<String>,
    #[serde(default)]
    pub(crate) annotations: Vec<CodeAnnotation>,
    #[serde(default)]
    pub(crate) modifiers: Vec<String>,
    // for TypeScript and JavaScript only, examples: `export default sample = createHello() `
    #[serde(default)]
    pub(crate) calls: Vec<CodeCall>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CodeFunction {
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) file_path: String,
    #[serde(default = "String::new")]
    pub(crate) package: String,
    pub(crate) return_type: String,
    #[serde(default)]
    pub(crate) multiple_returns: Vec<CodeProperty>,
    #[serde(default)]
    pub(crate) parameters: Vec<CodeProperty>,
    #[serde(default)]
    pub(crate) function_calls: Vec<CodeCall>,
    #[serde(default)]
    pub(crate) annotations: Vec<CodeAnnotation>,
    #[serde(default)]
    pub(crate) r#override: bool,
    #[serde(default)]
    pub(crate) modifiers: Vec<String>,
    // for example, Java can have Inner Class
    #[serde(default)]
    pub(crate) inner_structures: Vec<CodeDataStruct>,
    // for lambda or anonymous function inside function.
    #[serde(default)]
    pub(crate) inner_functions: Vec<CodeFunction>,
    pub(crate) position: CodePosition,
    pub(crate) extension: Option<JsonElement>,
    #[serde(default)]
    pub(crate) local_variables: Vec<CodeProperty>,
    pub(crate) is_constructor: Option<bool>, // todo: move to extension
    pub(crate) is_return_html: Option<bool>,
    pub(crate) body_hash: Option<i32>,
    #[serde(default = "FunctionType::function")]
    pub(crate) function_type: FunctionType,
    // a experimental API for code analysis, please carefully use it.
    // #[property]
    // expression: Expression,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JsonElement {
    pub(crate) data: Value,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CodeProperty {
    #[serde(default)]
    pub(crate) modifiers: Vec<String>,
    pub(crate) default_value: Option<String>,
    pub(crate) type_value: String,
    pub(crate) type_type: String,
    #[serde(default)]
    pub(crate) annotations: Vec<CodeAnnotation>,
    // for TypeScript and Parameter
    #[serde(default)]
    pub(crate) object_value: Vec<CodeProperty>,
    #[serde(default)]
    pub(crate) return_types: Vec<CodeProperty>,
    #[serde(default)]
    pub(crate) parameters: Vec<CodeProperty>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CodeAnnotation {
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) key_values: Vec<AnnotationKeyValue>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AnnotationKeyValue {
    pub(crate) key: String,
    pub(crate) value: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CodeCall {
    pub(crate) package: Option<String>,
    // for Java, it can be CreatorClass, lambda
    // for TypeScript, can be anonymous function, arrow function
    #[serde(default = "CallType::function")]
    pub(crate) call_type: CallType,
    // for Class/DataStruct, it's ClassName
    // for Function, it's empty
    #[serde(default)]
    pub(crate) node_name: String,
    #[serde(default)]
    pub(crate) function_name: String,
    #[serde(default)]
    pub(crate) parameters: Vec<CodeProperty>,
    pub(crate) position: CodePosition,
    // like "v1.Group", the v1 will be the Receiver
    // since 2.0.0-Beta.9
    #[serde(default)]
    pub(crate) origin_node_name: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum CallType {
    #[serde(rename = "field")]
    FIELD,
    #[serde(rename = "lambda")]
    LAMBDA,
    #[serde(rename = "arrow")]
    ARROW,
    #[serde(rename = "creator")]
    CREATOR,
    #[serde(rename = "function")]
    FUNCTION,
    // method come from parent
    #[serde(rename = "super")]
    SUPER,
    #[serde(rename = "same package")]
    SAME_PACKAGE,
    #[serde(rename = "self")]
    SELF,
    #[serde(rename = "chain")]
    CHAIN,
    #[serde(rename = "static")]
    STATIC,
}
impl CallType {
    fn function() -> Self { CallType::FUNCTION }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum FunctionType {
    Function,
    Block,
}
impl FunctionType {
    fn function() -> Self { FunctionType::Function }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CodePosition {
    pub(crate) start_line: i32,
    #[serde(default)]
    pub(crate) start_line_position: i32,
    pub(crate) stop_line: i32,
    #[serde(default)]
    pub(crate) stop_line_position: i32,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CodeImport {
    pub(crate) source: String,
    // todo: define for new usage
    #[serde(default)]
    pub(crate) as_name: String,
    // import UsageName from 'usage'
    // import AsSource as UsageName from 'source'
    pub(crate) usage_name: Vec<String>,
    pub(crate) scope: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CodeExport {
    pub(crate) name: String,
    pub(crate) source_file: String,
    pub(crate) data_type: Option<DataStructType>,
}


#[cfg(test)]
mod tests {
    use crate::model::CodeDataStruct;

    #[test]
    fn should_serialize_to_ds() {
        let json_ds = r#"[  {
    "NodeName": "DataMapAnalyserTest",
    "Module": "root:..:..",
    "Type": "CLASS",
    "Package": "org.archguard.scanner.analyser",
    "FilePath": "kotlin/org/archguard/scanner/analyser/DataMapAnalyserTest.kt",
    "Fields": [
      {
        "TypeType": "<UNKNOWN>",
        "TypeValue": "mockk<ArchGuardClient>{\nevery{saveDataStructure(any())}justruns\n}",
        "TypeKey": "mockClient",
        "Annotations": [],
        "Modifiers": [
          "private",
          "val"
        ],
        "Calls": []
      }
    ],
    "MultipleExtend": [],
    "Implements": [],
    "Functions": [
      {
        "Name": "tearDown",
        "Package": "org.archguard.scanner.analyser",
        "ReturnType": "kotlin.Unit",
        "MultipleReturns": [],
        "Parameters": [],
        "FunctionCalls": [
          {
            "Package": "io",
            "NodeName": "mockk",
            "FunctionName": "verify",
            "Parameters": [],
            "Position": {
              "StartLine": 24,
              "StartLinePosition": 8,
              "StopLine": 24,
              "StopLinePosition": 48
            }
          }
        ],
        "Annotations": [
          {
            "Name": "AfterEach",
            "KeyValues": []
          }
        ],
        "Modifiers": [],
        "InnerStructures": [],
        "InnerFunctions": [],
        "Position": {
          "StartLine": 22,
          "StartLinePosition": 3,
          "StopLine": 25,
          "StopLinePosition": 4
        },
        "LocalVariables": []
      }
    ],
    "InnerStructures": [],
    "Annotations": [],
    "FunctionCalls": [],
    "Parameters": [],
    "Imports": [
      {
        "Source": "io.mockk.clearAllMocks",
        "AsName": "clearAllMocks",
        "UsageName": []
      }
    ],
    "Exports": [],
    "Position": {
      "StartLine": 14,
      "StopLine": 26
    }
  }]
        "#;

        let ds: Vec<CodeDataStruct> = serde_json::from_str(json_ds).unwrap();
        assert_eq!(ds.len(), 1);
    }
}