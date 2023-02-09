import { writable, type Writable } from 'svelte/store';

// writable toast store
export const toastStore: Writable<Toast[]> = writable([]);

// toast interface
export interface Toast {
    message: string;
    type: string;
    position: string;
    duration: number;
}

// insert a toast
export function insertToast(toast: Toast) {
    toastStore.update((toasts) => {
        toasts.push(toast);
        return toasts;
    });
}

// insert a toast with a string message and a type
export function insertToastString(message: string, type: string,) {
    toastStore.update((toasts) => {

        let toast: Toast = {
            message: message,
            type: type,
            position: "top-right",
            duration: 3,
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