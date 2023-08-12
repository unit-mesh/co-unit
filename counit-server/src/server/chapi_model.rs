use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CodeDataStruct {
    // class and DataStruct Name
    // for TypeScript/JavaScript, if it is a variable, function, it will be named `default`
    node_name: String,
    module: String,
    #[serde(default = "DataStructType::default")]
    data_type: DataStructType, // You need to define DataStructType enum separately
    package: String,
    file_path: String,
    // todo: thinking of changing to property
    fields: Vec<CodeField>,
    multiple_extend: Vec<String>,
    implements: Vec<String>,
    #[serde(default)]
    extend: String,
    functions: Vec<CodeFunction>,
    inner_structures: Vec<CodeDataStruct>,
    annotations: Vec<CodeAnnotation>,
    function_calls: Vec<CodeCall>,
    #[deprecated(note = "looking for constructor method for SCALA")]
    parameters: Vec<CodeProperty>, // for Scala
    imports: Vec<CodeImport>,
    // in TypeScript, a file can export Function, Variable, Class, Interface
    // `export const baseURL = '/api'`
    exports: Vec<CodeExport>,
    // todo: select node use only imports
    extension: Option<JsonElement>, // You need to define JsonElement type separately
    position: CodePosition,
}


#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CodeField {
    type_type: String,
    type_value: String,
    type_key: String,
    annotations: Vec<CodeAnnotation>,
    modifiers: Vec<String>,
    // for TypeScript and JavaScript only, examples: `export default sample = createHello() `
    calls: Vec<CodeCall>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CodeFunction {
    name: String,
    #[serde(default)]
    file_path: String,
    package: String,
    return_type: String,
    multiple_returns: Vec<CodeProperty>,
    parameters: Vec<CodeProperty>,
    function_calls: Vec<CodeCall>,
    annotations: Vec<CodeAnnotation>,
    #[serde(default)]
    r#override: bool,
    modifiers: Vec<String>,
    // for example, Java can have Inner Class
    inner_structures: Vec<CodeDataStruct>,
    // for lambda or anonymous function inside function.
    inner_functions: Vec<CodeFunction>,
    position: CodePosition,
    extension: Option<JsonElement>,
    local_variables: Vec<CodeProperty>,
    is_constructor: Option<bool>, // todo: move to extension
    is_return_html: Option<bool>,
    body_hash: Option<i32>,
    #[serde(default = "FunctionType::function")]
    function_type: FunctionType,
    // a experimental API for code analysis, please carefully use it.
    // #[property]
    // expression: Expression,
}

#[derive(Serialize, Deserialize)]
struct JsonElement {
    data: Value,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CodeProperty {
    modifiers: Vec<String>,
    default_value: String,
    type_value: String,
    type_type: String,
    annotations: Vec<CodeAnnotation>,
    // for TypeScript and Parameter
    object_value: Vec<CodeProperty>,
    return_types: Vec<CodeProperty>,
    parameters: Vec<CodeProperty>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CodeAnnotation {
    name: String,
    key_values: Vec<AnnotationKeyValue>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AnnotationKeyValue {
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CodeCall {
    package: String,
    // for Java, it can be CreatorClass, lambda
    // for TypeScript, can be anonymous function, arrow function
    #[serde(default = "CallType::function")]
    call_type: CallType,
    // for Class/DataStruct, it's ClassName
    // for Function, it's empty
    node_name: String,
    function_name: String,
    parameters: Vec<CodeProperty>,
    position: CodePosition,
    // like "v1.Group", the v1 will be the Receiver
    // since 2.0.0-Beta.9
    #[serde(default)]
    origin_node_name: String,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
enum FunctionType {
    Function,
    Block,
}
impl FunctionType {
    fn function() -> Self { FunctionType::Function }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CodePosition {
    start_line: i32,
    #[serde(default)]
    start_line_position: i32,
    stop_line: i32,
    #[serde(default)]
    stop_line_position: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CodeImport {
    source: String,
    // todo: define for new usage
    #[serde(default)]
    as_name: String,
    // import UsageName from 'usage'
    // import AsSource as UsageName from 'source'
    usage_name: Vec<String>,
    scope: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CodeExport {
    name: String,
    source_file: String,
    data_type: Option<DataStructType>,
}


#[cfg(test)]
mod tests {
    use crate::server::chapi_model::CodeDataStruct;

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