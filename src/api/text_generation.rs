use actix_web::{post, web};
use log::info;
use rust_bert::{
    gpt_neo::{
        GptNeoConfigResources, GptNeoMergesResources, GptNeoModelResources, GptNeoVocabResources,
    },
    pipelines::{
        common::ModelType,
        text_generation::{TextGenerationConfig, TextGenerationModel},
    },
    resources::RemoteResource,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GenTextRequest {
    pub prefix: String,
    pub texts: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GenTextResponse {
    pub texts: Vec<String>,
}

#[post("/gen_text")]
pub async fn gen_text(gen_text_input: web::Json<GenTextRequest>) -> web::Json<GenTextResponse> {
    let request_id = uuid::Uuid::new_v4();
    info!("[{}] Started text generation.", request_id);

    let model_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoModelResources::GPT_NEO_2_7B,
    ));
    let config_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoConfigResources::GPT_NEO_2_7B,
    ));
    let vocab_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoVocabResources::GPT_NEO_2_7B,
    ));
    let merges_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoMergesResources::GPT_NEO_2_7B,
    ));

    let generate_config = TextGenerationConfig {
        model_type: ModelType::GPTNeo,
        model_resource,
        config_resource,
        vocab_resource,
        merges_resource,
        num_beams: 5,
        no_repeat_ngram_size: 2,
        max_length: 100,
        ..Default::default()
    };

    info!("[{}] Building model.", request_id);

    let blocking_task = 
        tokio::task::spawn_blocking(|| TextGenerationModel::new(generate_config));

    let model = blocking_task.await.unwrap().unwrap();
    
    info!("[{}] Generating results.", request_id);
    
    let prefix = gen_text_input.prefix.as_str();
    let output = model.generate(&gen_text_input.texts, Some(prefix));
    
    let mut results = Vec::new();
    
    for sentence in output {
        results.push(sentence);
    }

    info!("[{}] Finished generating texts.", request_id);

    web::Json(GenTextResponse { texts: results })
}
