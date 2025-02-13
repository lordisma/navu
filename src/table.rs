/// Table for the commands present in the application,
/// it will work as a bridge between the UI and the search thread, in a way that the search thread will update
/// it regularly with the commands found and the UI will render it.
/// 
/// This table will be shared between different threads, so it needs to provide a safe way to access and update it.

pub struct Table {
    
}