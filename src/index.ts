import common from './common';
import filesController from './filesController';

export interface Env {
	SHARED_API_KEY: string;
	FILES_BUCKET: R2Bucket;
}

export default {
	async fetch(
		request: Request,
		env: Env,
		_ctx: ExecutionContext
	): Promise<Response> {
		const url = new URL(request.url);
		if (url.pathname.startsWith('/files/')) {
			return await filesController(url, request, env);
		}
		return common.notFound();
	},
};
