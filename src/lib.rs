use worker::*;

mod files;

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    // TODO: Support GET
    // files::get_file("");

    // TODO: Support PUT
    // files::put_file("");

    // TODO: Support DELETE
    // files::delete_file("");

    Response::error("404s and heartbreaks", 404)
}
