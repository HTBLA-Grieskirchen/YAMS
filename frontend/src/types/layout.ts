import type {ReactElement} from 'react'
import type {NextPage} from 'next'

export type LayoutPage<P = {}, IP = P> = NextPage<P, IP> & {
    Layout?: Layout
}

export type NavigationPage<P = {}, IP = P> = NextPage<P, IP> & {
    NavPath?: () => ReactElement
    NavMenu?: () => ReactElement
}

export type Layout = ({children}: { children: ReactElement }) => ReactElement