use super::{
    super::{
        super::{
            commands::DBCommand,
            db_connect_cache::DBConnectCache,
        },
        db_core_state::DBCoreState,
        db_core_logic::DBCoreLogic,
    },
    DBCoreStateHandle,
};


impl DBCoreStateHandle {
    pub fn ready_handle(
        command: DBCommand,
        db_connect_cache: &mut DBConnectCache,
    ) -> Option<DBCoreState> {
        match command {
            DBCommand::Shutdown => {
                Some(DBCoreState::Shutdown)
            },
            DBCommand::CreateDBFile { 
                db_file_path, 
                migrations,
                response_target 
            } => {
                DBCoreLogic::create_db_file(
                    &db_file_path, 
                    migrations, 
                    response_target
                ) 
            },
            DBCommand::OpenDBConnect { 
                db_file_path, 
                response_target 
            } => {
                DBCoreLogic::open_db_connect(
                    &db_file_path, 
                    db_connect_cache, 
                    response_target
                ) 
            },
        }
    } 
}
