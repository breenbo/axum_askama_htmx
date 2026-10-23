# Usefull note learned in blood and tears

- Handlers MUST be ASYNC, even if nothing async inside
- in create_router in main.rs, use state:AppState, and not State(state), which is an extractor
- in handlers, USE State(state): State<Arc<AppState>> to extract the state
- Use Arc to have cheap copy/clone for AppState, sharing config and DbPool
- if form used in html, use Form(user): Form<RegisterFrom>> to extract the datas
