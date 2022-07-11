import { Env } from ".";
import common from "./common";

export default {
    getFile: async (
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
    },
    putFile: async (
        key: string,
        env: Env,
        request: Request
    ): Promise<Response> => {
        await env.FILES_BUCKET.put(key, request.body);
        return common.ok();
    },
    deleteFile: async (
        key: string,
        env: Env
    ): Promise<Response> => {
        await env.FILES_BUCKET.delete(key);
        return common.ok();
    }
}
