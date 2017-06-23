pub enum Command {
    Run(Data),
}

#[derive(Debug)]
pub struct Data {
    pub dir: String,
    pub config: Option<String>,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Config {
    pub url: String,
    pub token: String,
    pub service_type_id: i32,
    pub customer_id: i32,
    pub operators: i32,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    pub task_type: String,
    pub description: String,
    pub address: Option<String>,
    pub address_extra: Option<String>,
    pub city: String,
    pub dep_state: String,
    pub country: String,
    pub zip_code: Option<String>,
    pub lat: f32,
    pub lon: f32,
    pub reference: Option<String>,
    pub payment_type: String,
    pub payment_amount: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Service {
    pub service_type_id: i32,
    pub customer_id: i32,
    pub num_operators: i32,
    pub tasks: Vec<Task>,
}
