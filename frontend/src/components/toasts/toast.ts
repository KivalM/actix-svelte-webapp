import { writable, type Writable } from 'svelte/store';

// writable toast store

export const toastStore: Writable<Toast[]> = writable([]);


export interface Toast {
    message: string;
    type: string;
    position: string;
    duration: number;
}




export function insertToast(toast: Toast) {
    toastStore.update((toasts) => {
        toasts.push(toast);
        return toasts;
    });
}

export function insertToastString(message: string, type: string,) {

    console.log("inserting toast");

    toastStore.update((toasts) => {

        let toast: Toast = {
            message: message,
            type: type,
            position: "top-right",
            duration: 10,
        };

        toasts.push(toast);
        return toasts;
    });
}


// loop through the toasts and remove them after their duration
setInterval(() => {
    toastStore.update((toasts) => {
        toasts.forEach((toast) => {
            if (toast.duration > 0) {
                toast.duration -= 1;
            } else {
                toasts.shift();
            }
        });
        return toasts;
    });
}
    , 1000);