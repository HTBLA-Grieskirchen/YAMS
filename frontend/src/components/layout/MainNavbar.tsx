import { observer } from "mobx-react";
import React, { ReactElement, ReactNode } from "react";

const MainNavbar = observer((
    {children, NavbarPath, NavbarMenu}:
        { children: ReactNode, NavbarPath: () => ReactElement, NavbarMenu: () => ReactElement }
) => {
    return <>
        <div className="w-full navbar bg-base-200 shadow-md sticky top-0 z-40">
            <div className="flex-none [@media(min-width:1024px)]:hidden">
                <label htmlFor="main-drawer" className="btn btn-square btn-ghost">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                         className="inline-block w-6 h-6 stroke-current">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2"
                              d="M4 6h16M4 12h16M4 18h16"></path>
                    </svg>
                </label>
            </div>
            <div className="flex-1 px-2 py-0 breadcrumbs">
                <ul>
                    <NavbarPath/>
                </ul>
            </div>
            <div className="flex-none block">
                <ul className="menu menu-horizontal px-1 ">
                    <NavbarMenu/>
                </ul>
            </div>
        </div>

        {children}
    </>
})

export default MainNavbar

export const NavbarMenuEntry = observer((
    {disabled, children}:
        { disabled?: boolean, children?: ReactNode }
) => {
    return <li className={disabled ? "opacity-25 pointer-events-none" : ""}>
        {children}
    </li>
})
