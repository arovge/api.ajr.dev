import { Env } from ".";
import common from "./common"
import filesService from "./filesService";

export default (
    url: URL,
    request: Request,
    env: Env
): Promise<Response> => {
    const key = url.pathname.replace('/files/', '');
    if (request.method.toLowerCase() === 'get') {
        return filesService.getFile(key, env);
    }
    if (request.method.toLowerCase() === 'put') {
        if (common.isAuthorized(request, env)) {
            return filesService.putFile(key, env, request);
        } else {
            return common.forbidden();
        }
    }
    if (request.method.toLowerCase() === 'delete') {
        if (common.isAuthorized(request, env)) {
            return filesService.deleteFile(key, env);
        } else {
            return common.forbidden();
        }
    }
    return Promise.resolve(common.notFound());
}
