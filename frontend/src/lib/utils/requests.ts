// This function makes a simple post request to the rust backend
// we use json as the content type for our backend so everything is json here
// we also send the credentials so we can use cookies

export async function backendPost(url: string, body: object): Promise<Response> {

    let backendURL = "http://localhost:8000";
    // let backendURL = "http://127.0.0.1:8000";
    url = backendURL + url;  // add the backend url to the path
    const response = await fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(body),
        credentials: 'include', // allow send/recieve cookies
    });

    return response;

}

// This function makes a simple get request to the rust backend
// we use json as the content type for our backend so everything is json here
// we also send the credentials so we can use cookies
export async function backendGet(url: string): Promise<Response> {
    // log cookies
    console.log(document.cookie);

    let backendURL = "http://localhost:8000";
    url = backendURL + url;  // add the backend url to the path


    const response = await fetch(url, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include', // allow send/recieve cookies

    });

    return response;
}


