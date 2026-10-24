# Usefull note learned in blood and tears

- Handlers MUST be ASYNC, even if nothing async inside
- in create_router in main.rs, use state:AppState, and not State(state), which is an extractor
- in handlers, USE State(state): State<Arc<AppState>> to extract the state
- Use Arc to have cheap copy/clone for AppState, sharing config and DbPool
- if form used in html, use Form(user): Form<RegisterFrom>> to extract the datas
- Form extractor must always be the last in handlers (consume the body because implement FromRequest)

## AUTH

- id of logged in user stored using session: Session
- CurrentUser: use an extractor (function used in handlers) to get the logged-in state of the user everywhere, and call it in the handler. If return Ok, the value is passed to the handler, if Err, axum convert into http and handler not called. Extractor impl FromRequestPart (methods, url, headers, etc.), not the body
- axum handlers works on several threads, so Send + Sync in extractor is mandatory

## Routes

- RequireUser: redirect to /login if the user is not logged in (check with session), the handler using it is even not fired.
