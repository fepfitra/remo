type ResponseType<T = unknown> = {
	succeed: boolean;
	status: number;
	message: string;
	data: T;
};

async function get<T>(url: string): Promise<ResponseType<T>> {
	const response = await fetch(url, {
		method: "GET",
	});

	let resData: ResponseType<T>;
	try {
		const data = await response.json();
		resData = {
			succeed: response.ok,
			status: response.status,
			message: response.ok ? "OK" : (data.error || "Unknown error"),
			data: data as T,
		};
	} catch (error) {
		resData = {
			succeed: false,
			status: response.status,
			message: "Failed to parse response",
			data: null as any,
		};
	}

	if (!resData.succeed) {
		throw resData;
	}

	return resData;
}

async function post<T>(url: string, data?: BasicType): Promise<ResponseType<T>> {
	const response = await fetch(url, {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
		},
		body: JSON.stringify(data),
	});

	let resData: ResponseType<T>;
	try {
		const body = await response.json();
		resData = {
			succeed: response.ok,
			status: response.status,
			message: response.ok ? "OK" : (body.error || "Unknown error"),
			data: body as T,
		};
	} catch (error) {
		// Handle cases where response is not JSON (e.g. 204 No Content or 500 HTML)
		// For signout, it returns JSON, so this is catch-all for bad server errors.
		resData = {
			succeed: response.ok,
			status: response.status,
			message: response.ok ? "OK" : "Request failed",
			data: null as any,
		};
	}

	if (!resData.succeed) {
		throw resData;
	}

	return resData;
}

namespace api {
	export function getUserInfo() {
		return get<Model.User>("/api/user/me");
	}

	export function signin(username: string, password: string) {
		return post("/api/auth/signin", { username, password });
	}

	export function signup(username: string, password: string) {
		return post("/api/auth/signup", { username, password });
	}

	export function signout() {
		return post("/api/auth/signout");
	}

	export function checkUsernameUsable(username: string) {
		return get<boolean>("/api/user/checkusername?username=" + username);
	}

	export function checkPasswordValid(password: string) {
		return post<boolean>("/api/user/checkpassword", { password });
	}

	export function updateUserinfo(username?: string, password?: string, githubName?: string, wxUserId?: string) {
		return post("/api/user/update", {
			username,
			password,
			githubName,
			wxUserId,
		});
	}

	export function getMyMemos() {
		return get<Model.Memo[]>("/api/memo/all");
	}

	export function getMyDeletedMemos() {
		return get<Model.Memo[]>("/api/memo/deleted");
	}

	export function createMemo(content: string) {
		return post<Model.Memo>("/api/memo/new", { content });
	}

	export function getMemoById(id: string) {
		return get<Model.Memo>("/api/memo/?id=" + id);
	}

	export function hideMemo(memoId: string) {
		return post("/api/memo/hide", {
			memoId,
		});
	}

	export function restoreMemo(memoId: string) {
		return post("/api/memo/restore", {
			memoId,
		});
	}

	export function deleteMemo(memoId: string) {
		return post("/api/memo/delete", {
			memoId,
		});
	}

	export function updateMemo(memoId: string, content: string) {
		return post<Model.Memo>("/api/memo/update", { memoId, content });
	}

	export function getLinkedMemos(memoId: string) {
		return get<Model.Memo[]>("/api/memo/linked?memoId=" + memoId);
	}

	export function removeGithubName() {
		return post("/api/user/updategh", { githubName: "" });
	}

	export function getMyQueries() {
		return get<Model.Query[]>("/api/query/all");
	}

	export function createQuery(title: string, querystring: string) {
		return post<Model.Query>("/api/query/new", { title, querystring });
	}

	export function updateQuery(queryId: string, title: string, querystring: string) {
		return post<Model.Query>("/api/query/update", { queryId, title, querystring });
	}

	export function deleteQueryById(queryId: string) {
		return post("/api/query/delete", { queryId });
	}

	export function pinQuery(queryId: string) {
		return post("/api/query/pin", { queryId });
	}

	export function unpinQuery(queryId: string) {
		return post("/api/query/unpin", { queryId });
	}
}

export default api;
