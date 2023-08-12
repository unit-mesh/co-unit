use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeDataStruct {
    // class and DataStruct Name
    // for TypeScript/JavaScript, if it is a variable, function, it will be named `default`
    node_name: String,
    module: String,
    data_type: DataStructType, // You need to define DataStructType enum separately
    package: String,
    file_path: String,
    // todo: thinking of changing to property
    fields: Vec<CodeField>,
    multiple_extend: Vec<String>,
    implements: Vec<String>,
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
    extension: JsonElement, // You need to define JsonElement type separately
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct CodeFunction {
    name: String,
    file_path: String,
    package: String,
    return_type: String,
    multiple_returns: Vec<CodeProperty>,
    parameters: Vec<CodeProperty>,
    function_calls: Vec<CodeCall>,
    annotations: Vec<CodeAnnotation>,
    r#override: bool,
    modifiers: Vec<String>,
    // for example, Java can have Inner Class
    inner_structures: Vec<CodeDataStruct>,
    // for lambda or anonymous function inside function.
    inner_functions: Vec<CodeFunction>,
    position: CodePosition,
    extension: JsonElement,
    local_variables: Vec<CodeProperty>,
    is_constructor: bool, // todo: move to extension
    is_return_html: bool,
    body_hash: i32,
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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct CodeAnnotation {
    name: String,
    key_values: Vec<AnnotationKeyValue>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnotationKeyValue {
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CodeCall {
    package: String,
    // for Java, it can be CreatorClass, lambda
    // for TypeScript, can be anonymous function, arrow function
    call_type: CallType,
    // for Class/DataStruct, it's ClassName
    // for Function, it's empty
    node_name: String,
    function_name: String,
    parameters: Vec<CodeProperty>,
    position: CodePosition,
    // like "v1.Group", the v1 will be the Receiver
    // since 2.0.0-Beta.9
    origin_node_name: String,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
enum FunctionType {
    Function,
    Block,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CodePosition {
    start_line: i32,
    start_line_position: i32,
    stop_line: i32,
    stop_line_position: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CodeImport {
    source: String,
    // todo: define for new usage
    as_name: String,
    // import UsageName from 'usage'
    // import AsSource as UsageName from 'source'
    usage_name: Vec<String>,
    scope: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CodeExport {
    name: String,
    source_file: String,
    data_type: DataStructType,
}

