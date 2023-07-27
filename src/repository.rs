use gloo::net::http::Request;
use crate::model::employee::Employee;

pub struct Repository;
impl Repository{
    pub async fn get_employees() -> Vec<Employee>{
        const API_URL: &str= "";
        let fetched_employees: Vec<Employee> = Request::get(API_URL)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        fetched_employees
    }
}