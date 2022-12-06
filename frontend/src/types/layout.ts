import type {ReactElement} from 'react'
import type {NextPage} from 'next'

export type NextLayoutPage<P = {}, IP = P> = NextPage<P, IP> & {
    Layout?: Layout
}

export type Layout = ({children}: { children: ReactElement }) => ReactElement