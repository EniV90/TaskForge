// Defining Api utils

import axios, { AxiosResponse } from "axios";

async function handleRequest<T, X>(
  promise: Promise<AxiosResponse<X>>,
  expectedResponse: number
) {
  let response: AxiosResponse<X>;
  try {
    response = await promise;
  } catch (error) {
    return {
      status: 500,
      error: "Network Error",
      data: JSON.stringify(error),
    };
  }

  if (response.status === expectedResponse) {
    return {
      status: response.status,
      data: response.data as X,
    };
  } else {
    return {
      status: response.status,
      error: `expected status ${expectedResponse} got ${response.status}`,
      data: response.data as X,
    };
  }
}

// using handleRequest function to define a function that executes a post request

export async function postCall<T, X>(
  url: string,
  body: T,
  expectedResponse: number
) {
  let response = axios.post<X | string>(url, body, {
    headers: {
      "Content-Type": "application/json",
      token: "jwt",
    },
    validateStatus: () => true,
  });
  return handleRequest(response, expectedResponse);
}

// getCall function, similar to the postCall function
export async function getCall<X>(url: string, expectedResponse: number) {
  let response = axios.get<X | string>(url, {
    headers: {
      "Content-Type": "application/json",
      token: "jwt",
    },
    validateStatus: () => true,
  });
  return handleRequest(response, expectedResponse);
}

// deleteCall
export async function deleteCall<X>(url: string, expectedResponse: number) {
  let response = axios.delete<X | string>(url, {
    headers: {
      "Content-Type": "application/json",
      token: "jwt",
    },
    validateStatus: () => true,
  });
  return handleRequest(response, expectedResponse);
}

// patchCall
export async function patchCall<T, X>(
  url: string,
  body: T,
  expectedResponse: number
) {
  let response = axios.patch<X | string>(url, body, {
    headers: {
      "Content-Type": "app,ication/json",
      token: "jwt",
    },
    validateStatus: () => true,
  });
  return handleRequest(response, expectedResponse);
}

// putCall
export async function putCall<T, X>(
  url: string,
  body: T,
  expectedResponse: number
) {
  let response = axios.put<X | string>(url, body, {
    headers: {
      "Content-Type": "application/json",
      token: "jwt",
    },
    validateStatus: () => true,
  });
  return handleRequest(response, expectedResponse);
}
