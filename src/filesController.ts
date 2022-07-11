import { Env } from ".";
import common from "./common"

export default (
    url: URL,
    request: Request,
    env: Env
): Promise<Response> => {
    const key = url.pathname.replace('/files/', '');
    if (request.method.toLowerCase() === 'get') {
        return getFile(key, env);
    }
    if (request.method.toLowerCase() === 'put') {
        if (common.isAuthorized(request, env)) {
            return putFile(key, env, request);
        } else {
            return common.forbidden();
        }
    }
    if (request.method.toLowerCase() === 'delete') {
        if (common.isAuthorized(request, env)) {
            return deleteFile(key, env);
        } else {
            return common.forbidden();
        }
    }
    return Promise.resolve(common.notFound());
}

const getFile = async (
    key: string,
    env: Env
): Promise<Response> => {
    const file = await env.FILES_BUCKET.get(key);

    if (file === null) {
        return common.notFound();
    }

    const headers = new Headers();
    file.writeHttpMetadata(headers);
    headers.set('etag', file.httpEtag);

    return new Response(file.body, { headers });
};

const putFile = async (
    key: string,
    env: Env,
    request: Request
): Promise<Response> => {
    await env.FILES_BUCKET.put(key, request.body);
    return common.ok();
};

const deleteFile = async (
    key: string,
    env: Env
): Promise<Response> => {
    await env.FILES_BUCKET.delete(key);
    return common.ok();
};
