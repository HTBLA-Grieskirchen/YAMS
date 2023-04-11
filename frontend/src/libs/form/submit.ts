import {useLocalObservable} from "mobx-react";

export function useSubmissionState() {
    return useLocalObservable(() => ({
        submitted: false,
        submit() {
            this.submitted = true
        },
        clear() {
            this.submitted = false
        }
    }))
}

export type SubmissionState = ReturnType<typeof useSubmissionState>
