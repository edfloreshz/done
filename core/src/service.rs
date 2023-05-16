use crate::database::Database;
use crate::diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use crate::models::{QueryableList, QueryableTask};
use crate::schema::lists::dsl::*;
use crate::schema::tasks::dsl::*;
use anyhow::Context;
use proto_rust::provider::provider_server::Provider;
use proto_rust::provider::{List, ProviderResponse, Task};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct LocalService {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
}

#[tonic::async_trait]
impl Provider for LocalService {
    async fn get_task(
        &self,
        request: Request<String>,
    ) -> Result<Response<ProviderResponse>, tonic::Status> {
        tracing::info!("Request received: {request:?}");
        let id = request.into_inner();
        let mut response = ProviderResponse::default();

        let send_request = || -> anyhow::Result<Task> {
            let result: QueryableTask = tasks
                .find(id)
                .first(&mut Database::establish_connection()?)
                .context("Failed to fetch list of tasks.")?;
            Ok(result.into())
        };

        match send_request() {
            Ok(value) => {
                response.task = Some(value);
                response.successful = true;
                response.message = "Task fetched successfully.".to_string()
            }
            Err(err) => response.message = err.to_string(),
        }
        Ok(Response::new(response))
    }

    type GetTasksStream = ReceiverStream<Result<ProviderResponse, Status>>;

    async fn get_tasks(
        &self,
        request: Request<String>,
    ) -> Result<Response<Self::GetTasksStream>, Status> {
        tracing::info!("Request received: {request:?}");
        let (tx, rx) = tokio::sync::mpsc::channel(4);
        let id = request.into_inner();

        let send_request = || -> anyhow::Result<Vec<Task>> {
            let result: Vec<QueryableTask> = tasks
                .filter(parent.eq(id))
                .load::<QueryableTask>(&mut Database::establish_connection()?)?;
            let results: Vec<Task> = result.iter().map(|t| t.clone().into()).collect();
            Ok(results)
        };

        tokio::spawn(async move {
            match send_request() {
                Ok(value) => {
                    for task in &value[..] {
                        let response = ProviderResponse {
                            successful: true,
                            message: "Task fetched successfully".to_string(),
                            task: Some(task.clone()),
                            ..Default::default()
                        };
                        tx.send(Ok(response)).await.unwrap();
                    }
                }
                Err(err) => tracing::error!("{err}"),
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn create_task(
        &self,
        request: Request<Task>,
    ) -> Result<Response<ProviderResponse>, Status> {
        tracing::info!("Request received: {request:?}");
        let task = request.into_inner();
        let mut response = ProviderResponse::default();

        let send_request = || -> anyhow::Result<()> {
            let queryable_task: QueryableTask = task.clone().into();

            diesel::insert_into(tasks)
                .values(&queryable_task)
                .execute(&mut Database::establish_connection()?)?;

            Ok(())
        };

        match send_request() {
            Ok(()) => {
                response.task = Some(task);
                response.successful = true;
                response.message = "Task added successfully.".to_string()
            }
            Err(err) => response.message = err.to_string(),
        }
        Ok(Response::new(response))
    }

    async fn update_task(
        &self,
        request: Request<Task>,
    ) -> Result<Response<ProviderResponse>, Status> {
        tracing::info!("Request received: {request:?}");
        let task = request.into_inner();
        let mut response = ProviderResponse::default();

        let send_request = || -> anyhow::Result<Task> {
            let original_task = task.clone();
            let queryable_task: QueryableTask = task.into();

            diesel::update(tasks.filter(id_task.eq(queryable_task.id_task.clone())))
                .set((
                    id_task.eq(queryable_task.id_task),
                    parent.eq(queryable_task.parent),
                    title.eq(queryable_task.title),
                    favorite.eq(queryable_task.favorite),
                    today.eq(queryable_task.today),
                    status.eq(queryable_task.status),
                    priority.eq(queryable_task.priority),
                    sub_tasks.eq(queryable_task.sub_tasks),
                    tags.eq(queryable_task.tags),
                    notes.eq(queryable_task.notes),
                    completion_date.eq(queryable_task.completion_date),
                    deletion_date.eq(queryable_task.deletion_date),
                    due_date.eq(queryable_task.due_date),
                    reminder_date.eq(queryable_task.reminder_date),
                    recurrence.eq(queryable_task.recurrence),
                    created_date_time.eq(queryable_task.created_date_time),
                    last_modified_date_time.eq(queryable_task.last_modified_date_time),
                ))
                .execute(&mut Database::establish_connection()?)
                .context("Failed to update task.")?;

            Ok(original_task)
        };

        match send_request() {
            Ok(task) => {
                response.task = Some(task);
                response.successful = true;
                response.message = "Task updated successfully".to_string()
            }
            Err(err) => response.message = err.to_string(),
        }
        Ok(Response::new(response))
    }

    async fn delete_task(
        &self,
        request: Request<String>,
    ) -> Result<Response<ProviderResponse>, Status> {
        tracing::info!("Request received: {request:?}");
        let id = request.into_inner();
        let mut response = ProviderResponse::default();

        let send_request = || -> anyhow::Result<()> {
            diesel::delete(tasks.filter(id_task.eq(id)))
                .execute(&mut Database::establish_connection()?)?;

            Ok(())
        };

        match send_request() {
            Ok(()) => {
                response.successful = true;
                response.message = "Task removed successfully.".to_string()
            }
            Err(err) => response.message = err.to_string(),
        }
        Ok(Response::new(response))
    }

    async fn get_list(
        &self,
        request: Request<String>,
    ) -> Result<Response<ProviderResponse>, Status> {
        tracing::info!("Request received: {request:?}");
        let id = request.into_inner();
        let mut response = ProviderResponse::default();

        let send_request = || -> anyhow::Result<List> {
            let result: QueryableList = lists
                .find(id)
                .first(&mut Database::establish_connection()?)?;
            Ok(result.into())
        };

        match send_request() {
            Ok(value) => {
                response.list = Some(value);
                response.successful = true;
                response.message = "List fetched succesfully.".to_string()
            }
            Err(err) => response.message = err.to_string(),
        }
        Ok(Response::new(response))
    }

    type GetListsStream = ReceiverStream<Result<ProviderResponse, Status>>;

    async fn get_lists(
        &self,
        request: Request<()>,
    ) -> Result<Response<Self::GetListsStream>, Status> {
        tracing::info!("Request received: {request:?}");
        let (tx, rx) = tokio::sync::mpsc::channel(4);

        let send_request = || -> anyhow::Result<Vec<List>> {
            let results = lists.load::<QueryableList>(&mut Database::establish_connection()?)?;

            let results: Vec<List> = results.iter().map(|t| t.clone().into()).collect();
            Ok(results)
        };

        let mut response = ProviderResponse::default();

        tokio::spawn(async move {
            match send_request() {
                Ok(value) => {
                    response.successful = true;
                    for list in &value[..] {
                        let response = ProviderResponse {
                            list: Some(list.clone()),
                            successful: true,
                            message: "List fetched succesfully.".to_string(),
                            ..Default::default()
                        };
                        tx.send(Ok(response)).await.unwrap();
                    }
                }
                Err(err) => response.message = err.to_string(),
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn get_list_ids(
        &self,
        request: Request<()>,
    ) -> Result<Response<ProviderResponse>, Status> {
        tracing::info!("Request received: {request:?}");
        let send_request = || -> anyhow::Result<Vec<String>> {
            let result: Vec<String> = lists
                .select(id_list)
                .load::<String>(&mut Database::establish_connection()?)
                .context("Failed to fetch list of tasks.")?;
            Ok(result)
        };

        let mut response = ProviderResponse::default();

        match send_request() {
            Ok(result) => {
                response.successful = true;
                response.lists = result;
            }
            Err(_) => response.message = "Failed to fetch list of tasks".to_string(),
        }

        Ok(Response::new(response))
    }

    async fn create_list(
        &self,
        request: Request<List>,
    ) -> Result<Response<ProviderResponse>, Status> {
        tracing::info!("Request received: {request:?}");
        let list = request.into_inner();
        let mut response = ProviderResponse::default();

        let send_request = || -> anyhow::Result<()> {
            let list: QueryableList = list.into();

            diesel::insert_into(lists)
                .values(&list)
                .execute(&mut Database::establish_connection()?)?;

            Ok(())
        };

        match send_request() {
            Ok(()) => {
                response.list = None;
                response.successful = true;
                response.message = "List added succesfully".to_string()
            }
            Err(err) => response.message = err.to_string(),
        }
        Ok(Response::new(response))
    }

    async fn update_list(
        &self,
        request: Request<List>,
    ) -> Result<Response<ProviderResponse>, Status> {
        tracing::info!("Request received: {request:?}");
        let list = request.into_inner();
        let mut response = ProviderResponse::default();

        let send_request = || -> anyhow::Result<()> {
            let list: QueryableList = list.into();

            diesel::update(lists.filter(id_list.eq(list.id_list.clone())))
                .set((name.eq(list.name.clone()), icon_name.eq(list.icon_name)))
                .execute(&mut Database::establish_connection()?)
                .context("Failed to update list.")?;

            Ok(())
        };

        match send_request() {
            Ok(()) => {
                response.list = None;
                response.successful = true;
                response.message = "List updated succesfully.".to_string()
            }
            Err(err) => response.message = err.to_string(),
        }
        Ok(Response::new(response))
    }

    async fn delete_list(
        &self,
        request: Request<String>,
    ) -> Result<Response<ProviderResponse>, Status> {
        tracing::info!("Request received: {request:?}");
        let id = request.into_inner();
        let mut response = ProviderResponse::default();

        let send_request = || -> anyhow::Result<()> {
            diesel::delete(lists.filter(id_list.eq(id)))
                .execute(&mut Database::establish_connection()?)?;
            Ok(())
        };

        match send_request() {
            Ok(()) => {
                response.successful = true;
                response.message = "List removed succesfully.".to_string()
            }
            Err(err) => response.message = err.to_string(),
        }
        Ok(Response::new(response))
    }

    type GetTasksFromListStream = ReceiverStream<Result<ProviderResponse, Status>>;

    async fn get_tasks_from_list(
        &self,
        request: Request<String>,
    ) -> Result<Response<Self::GetTasksFromListStream>, Status> {
        tracing::info!("Request received: {request:?}");
        let (tx, rx) = tokio::sync::mpsc::channel(4);
        let id = request.into_inner();

        let send_request = || -> anyhow::Result<Vec<Task>> {
            let result: Vec<QueryableTask> = tasks
                .filter(parent.eq(id))
                .load::<QueryableTask>(&mut Database::establish_connection()?)
                .context("Failed to fetch list of tasks.")?;
            let results: Vec<Task> = result.iter().map(|t| t.clone().into()).collect();
            Ok(results)
        };

        let mut response = ProviderResponse::default();

        tokio::spawn(async move {
            match send_request() {
                Ok(value) => {
                    response.successful = true;
                    for task in &value[..] {
                        let response = ProviderResponse {
                            task: Some(task.clone()),
                            successful: true,
                            message: "Task fetched successfully".to_string(),
                            ..Default::default()
                        };
                        tx.send(Ok(response)).await.unwrap();
                    }
                }
                Err(err) => response.message = err.to_string(),
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn get_task_ids_from_list(
        &self,
        request: Request<String>,
    ) -> Result<Response<ProviderResponse>, Status> {
        tracing::info!("Request received: {request:?}");
        let send_request = || -> anyhow::Result<Vec<String>> {
            let result: Vec<String> = tasks
                .select(id_task)
                .filter(parent.eq(request.into_inner()))
                .load::<String>(&mut Database::establish_connection()?)
                .context("Failed to fetch list of tasks.")?;
            Ok(result)
        };

        let mut response = ProviderResponse::default();

        match send_request() {
            Ok(result) => {
                response.successful = true;
                response.tasks = result;
            }
            Err(_) => response.message = "Failed to fetch list of tasks".to_string(),
        }

        Ok(Response::new(response))
    }

    async fn get_task_count_from_list(
        &self,
        request: Request<String>,
    ) -> Result<Response<ProviderResponse>, Status> {
        tracing::info!("Request received: {request:?}");
        let id = request.into_inner();
        let mut response = ProviderResponse::default();

        let send_request = || -> anyhow::Result<i64> {
            let count: i64 = tasks
                .filter(id_task.eq(id))
                .count()
                .get_result(&mut Database::establish_connection()?)?;
            Ok(count)
        };

        match send_request() {
            Ok(value) => {
                response.count = value;
                response.successful = true;
            }
            Err(err) => response.message = err.to_string(),
        }
        Ok(Response::new(response))
    }
}
