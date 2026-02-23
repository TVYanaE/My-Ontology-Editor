use super::{
    super::{
        super::{
            events::DBCommand,
        },
        db_core_state::DBCoreState,
        db_core_logic::DBCoreLogic,
    },
    DBCoreStateHandle,
};


impl DBCoreStateHandle {
    pub fn ready_handle(
        command: DBCommand,
    ) -> Option<DBCoreState> {
        match command {
            DBCommand::Shutdown => {
                Some(DBCoreState::Shutdown)
            },
            DBCommand::CreateDBFile { 
                db_file_path, 
                migration,
                response_target 
            } => {
                DBCoreLogic::create_db_file(&db_file_path, migration, response_target) 
            },
        }
    } 
}
