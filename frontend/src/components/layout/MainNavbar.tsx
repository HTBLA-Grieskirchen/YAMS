import { observer } from "mobx-react";
import React, { ReactElement, ReactNode } from "react";
import { Navbar, NavbarBrand, NavbarContent, NavbarItem, Link, Button } from "@heroui/react";
import { Menu } from "lucide-react";

const MainNavbar = observer((
    {children, NavbarPath, NavbarMenu}:
        { children: ReactNode, NavbarPath: () => ReactElement, NavbarMenu: () => ReactElement }
) => {
    return <>
        <Navbar isBordered className="bg-background/70 backdrop-blur-md sticky top-0 z-40" maxWidth="full">
            <NavbarContent className="lg:hidden" justify="start">
                <label htmlFor="main-drawer" className="cursor-pointer p-2">
                    <Menu className="w-6 h-6" />
                </label>
            </NavbarContent>

            <NavbarContent className="flex-1 px-2" justify="start">
                <div className="flex items-center space-x-2 text-sm overflow-hidden">
                    <NavbarPath/>
                </div>
            </NavbarContent>

            <NavbarContent className="flex-none block" justify="end">
                <div className="flex items-center gap-4">
                    <NavbarMenu/>
                </div>
            </NavbarContent>
        </Navbar>

        {children}
    </>
})

export default MainNavbar

export const NavbarMenuEntry = observer((
    {disabled, children}:
        { disabled?: boolean, children?: ReactNode }
) => {
    return <NavbarItem isActive={!disabled} className={disabled ? "opacity-25 pointer-events-none" : ""}>
        {children}
    </NavbarItem>
})
