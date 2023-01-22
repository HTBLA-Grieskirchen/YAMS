import {observer} from "mobx-react";
import {ReactNode, useEffect, useState} from "react";
import {useStore} from "../../stores";
import Link from "next/link";
import paths from "../../util/paths";
import {autorun} from "mobx";

const MainMenu = observer((
    {children}:
        { children: ReactNode }
) => {
    return <div className="drawer drawer-mobile">
        <input id="main-drawer" type="checkbox" className="drawer-toggle"/>
        <div className="drawer-content flex flex-none flex-col">
            <div className="w-full navbar bg-base-300">
                <div className="flex-none [@media(min-width:1024px)]:hidden">
                    <label htmlFor="main-drawer" className="btn btn-square btn-ghost">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                             className="inline-block w-6 h-6 stroke-current">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2"
                                  d="M4 6h16M4 12h16M4 18h16"></path>
                        </svg>
                    </label>
                </div>
                <div className="flex-1 px-2 mx-2">Navbar Title</div>
                <div className="flex-none hidden lg:block">
                    <ul className="menu menu-horizontal">

                        <li><a>Navbar Item 1</a></li>
                        <li><a>Navbar Item 2</a></li>
                    </ul>
                </div>
            </div>

            {children}
        </div>
        <div className="drawer-side">
            <label htmlFor="main-drawer" className="drawer-overlay"></label>
            <aside className="bg-base-200 text-base-content w-fit h-full">
                <ul className="sticky top-0 bg-base-200 menu menu-compact w-full p-4 pb-0 z-20">
                    <li>
                        <Link href={paths.home}>
                            <a>
                                <i className="fa-regular fa-house"/>
                                Home
                            </a>
                        </Link>
                    </li>
                    <li></li>
                </ul>

                <ul className="menu menu-compact pb-0 p-0 px-4 w-full">
                    <MainMenuItems/>
                </ul>

                <div className="grid gap-4 sticky bottom-0 pb-4 bg-base-200">
                    <ul className="menu menu-compact p-0 px-4 w-full">
                        <li></li>
                        <li className="menu-title">
                            <span>Customization</span>
                        </li>
                        <li className="disabled">
                            <div className="flex flex-row">
                                <ThemePicker key={"theme"}/>

                                <LanguagePicker key={"language"}/>
                            </div>
                        </li>
                    </ul>


                    <div className="footer px-8">
                        <MainMenuLogo/>
                    </div>
                </div>
            </aside>
        </div>
    </div>
})
export default MainMenu

const MainMenuItems = observer(() => {
    return <>
        <li><a>Sidebar Item 1</a></li>
        <li><a>Sidebar Item 2</a></li>
        <li><a>Sidebar Item 1</a></li>
        <li><a>Sidebar Item 2</a></li>
        <li><a>Sidebar Item 1</a></li>
        <li><a>Sidebar Item 2</a></li>
        <li><a>Sidebar Item 1</a></li>
        <li><a>Sidebar Item 2</a></li>
        <li><a>Sidebar Item 1</a></li>
        <li><a>Sidebar Item 2</a></li>
        <li><a>Sidebar Item 1</a></li>
        <li><a>Sidebar Item 2</a></li>
        <li><a>Sidebar Item 1</a></li>
        <li><a>Sidebar Item 2</a></li>
        <li><a>Sidebar Item 1</a></li>
        <li><a>Sidebar Item 2</a></li>
        <li><a>Sidebar Item 1</a></li>
        <li><a>Sidebar Item 2</a></li>
        <li><a>Sidebar Item 1</a></li>
        <li><a>Sidebar Item 2</a></li>
        <li><a>Sidebar Item 1</a></li>
        <li><a>Sidebar Item 2</a></li>
    </>
})

const MainMenuLogo = observer(() => {
    return <div>
        <svg width="50" height="50" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fillRule="evenodd"
             clipRule="evenodd" className="fill-current">
            <path
                d="M22.672 15.226l-2.432.811.841 2.515c.33 1.019-.209 2.127-1.23 2.456-1.15.325-2.148-.321-2.463-1.226l-.84-2.518-5.013 1.677.84 2.517c.391 1.203-.434 2.542-1.831 2.542-.88 0-1.601-.564-1.86-1.314l-.842-2.516-2.431.809c-1.135.328-2.145-.317-2.463-1.229-.329-1.018.211-2.127 1.231-2.456l2.432-.809-1.621-4.823-2.432.808c-1.355.384-2.558-.59-2.558-1.839 0-.817.509-1.582 1.327-1.846l2.433-.809-.842-2.515c-.33-1.02.211-2.129 1.232-2.458 1.02-.329 2.13.209 2.461 1.229l.842 2.515 5.011-1.677-.839-2.517c-.403-1.238.484-2.553 1.843-2.553.819 0 1.585.509 1.85 1.326l.841 2.517 2.431-.81c1.02-.33 2.131.211 2.461 1.229.332 1.018-.21 2.126-1.23 2.456l-2.433.809 1.622 4.823 2.433-.809c1.242-.401 2.557.484 2.557 1.838 0 .819-.51 1.583-1.328 1.847m-8.992-6.428l-5.01 1.675 1.619 4.828 5.011-1.674-1.62-4.829z"></path>
        </svg>
        <p>Energetik Sabine Petschl<br/>Wohlfühlen für Mensch und Tier</p>
    </div>
})


type ThemeDefinition = {
    id: string,
    name: string,
}
const knownThemes = [
    {id: "light", name: "Light"},
    {id: "dark", name: "Dark"}
]

const ThemePicker = observer(() => {
    return <div className="dropdown dropdown-top text-base-content" title="Change Theme">
        <div className="btn btn-sm gap-1 normal-case btn-ghost" tabIndex={0}>
            <i className="fa-solid fa-swatchbook inline-block text-lg stroke-current md:text-xl align-bottom"/>
            <span className="hidden md:inline ml-1">Theme</span>
            <i className="fa-solid fa-angle-up ml-1 hidden text-sm fill-current opacity-60 sm:inline-block"/>
        </div>
        <div
            className="dropdown-content bg-base-200 text-base-content rounded-t-box rounded-b-box top-px max-h-96 h-fit w-48 overflow-y-auto shadow-2xl">
            <div className="grid grid-cols gap-3 p-3" tabIndex={0}>
                <ThemePreview theme={undefined}/>
                {knownThemes.map((theme) => <ThemePreview key={theme.id} theme={theme}/>)}
            </div>
        </div>
    </div>
})

const ThemePreview = observer((
    {theme}:
        { theme: ThemeDefinition | undefined }
) => {
    const store = useStore()

    const [selected, setSelected] = useState(false)
    const [dark, setDark] = useState(false)

    useEffect(() => {
        const deregister = autorun(() => {
            setSelected(store.settingsStore.theme == theme?.id)
        })

        return () => deregister()
    })

    useEffect(() => {
        if (window.matchMedia && theme === undefined) {
            const media = window.matchMedia('(prefers-color-scheme: dark)')
            media.addEventListener('change', e => setDark(e.matches))
            setDark(media.matches)
        }
    }, [])

    return <div className={`overflow-hidden rounded-lg ring-offset-2 ring-offset-base-200 ring-base-content 
        ${selected ? "ring" : ""}`}
                onClick={e => (store.settingsStore.theme != theme?.id) && store.settingsStore.setTheme(theme?.id ?? null)}>
        <div className={`bg-base-100 text-base-content w-full font-sans ${!selected ? "cursor-pointer" : ""}`}
             data-theme={theme?.id ?? (dark ? knownThemes[1].id : knownThemes[0].id)}>
            <div className="grid grid-cols-5 grid-rows-3">
                <div className="col-span-5 row-span-3 row-start-1 flex gap-1 py-2 px-2">
                    <div className="flex-grow text-sm font-bold">{theme?.name ?? "System"}</div>
                    <div className="flex flex-shrink-0 flex-wrap gap-1">
                        <div className="bg-primary w-2 rounded"/>
                        <div className="bg-secondary w-2 rounded"/>
                        <div className="bg-accent w-2 rounded"/>
                        <div className="bg-neutral w-2 rounded"/>
                    </div>
                </div>
            </div>
        </div>
    </div>
})


type LanguageDefinition = {
    id: string,
    name: string,
    flag: string
}
const knownLanguages = [
    {id: "en_US", name: "English", flag: "🇺🇸"},
    {id: "de_DE", name: "Deutsch", flag: "🇩🇪"}
]

const LanguagePicker = observer(() => {
    return <div className="dropdown dropdown-top dropdown-end text-base-content" title="Change Language">
        <div className="btn btn-sm gap-1 normal-case text-base-content btn-ghost" tabIndex={0}>
            <i className="fa-solid fa-language inline-block text-lg stroke-current md:text-xl align-bottom"/>
            <i className="fa-solid fa-angle-up ml-1 hidden text-md fill-current opacity-60 sm:inline-block"/>
        </div>
        <div
            className="dropdown-content bg-base-200 text-base-content rounded-t-box rounded-b-box top-px max-h-96 h-fit w-48 overflow-y-auto shadow-2xl">
            <ul className="menu menu-compact gap-1 p-3">
                {knownLanguages.map((lang) => <LanguagePreview key={lang.id} language={lang}/>)}
            </ul>
        </div>
    </div>
})

const LanguagePreview = observer((
    {language}:
        { language: LanguageDefinition }
) => {
    const store = useStore()

    const [selected, setSelected] = useState(false)

    useEffect(() => {
        const deregister = autorun(() => {
            setSelected(store.settingsStore.language == language.id)
        })

        return () => deregister()
    })

    return <li>
        <button className={`flex ${selected ? "active cursor-default" : ""}`}
                onClick={e => store.settingsStore.language != language.id && store.settingsStore.setLanguage(language.id)}>
            <span className="text-md">{language.flag}</span>
            <span className="flex flex-1 justify-between">{language.name}</span>
        </button>
    </li>
})
