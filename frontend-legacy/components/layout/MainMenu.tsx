import { observer } from "mobx-react";
import { ReactNode, useEffect, useState } from "react";
import { useStore } from "../../stores";
import Link from "next/link";
import paths from "../../util/paths";
import { autorun } from "mobx";
import { useRouter } from "next/router";
import {
  Button,
  Dropdown,
  DropdownTrigger,
  DropdownMenu,
  DropdownItem,
  Card,
  CardBody,
} from "@heroui/react";
import { Home, Palette, Languages, MoreVertical } from "lucide-react";

const MainMenu = observer(
  ({
    children,
    entries,
  }: {
    children: ReactNode;
    entries?: MainMenuEntries;
  }) => {
    const router = useRouter();

    return (
      <div className="flex h-screen overflow-hidden">
        <input id="main-drawer" type="checkbox" className="hidden peer" />

        {/* Sidebar */}
        <aside className="fixed inset-y-0 left-0 z-50 w-64 bg-background border-r transition-transform -translate-x-full lg:translate-x-0 lg:static lg:inset-0 peer-checked:translate-x-0">
          <div className="flex flex-col h-full overflow-y-auto px-4 py-6">
            <div className="mb-8 px-2">
              <Link href={paths.home} passHref legacyBehavior>
                <Button
                  fullWidth
                  variant={router.pathname === "/" ? "solid" : "light"}
                  color={router.pathname === "/" ? "primary" : "default"}
                  className="justify-start gap-3"
                  as="a"
                >
                  <Home size={20} />
                  Home
                </Button>
              </Link>
            </div>

            <div className="flex-1 space-y-8">
              {entries && <MainMenuItems entries={entries} />}
            </div>

            <div className="mt-auto pt-6 border-t space-y-4">
              <div className="flex gap-2">
                <ThemePicker />
                <LanguagePicker />
              </div>
              <MainMenuLogo />
            </div>
          </div>
        </aside>

        {/* Overlay */}
        <label
          htmlFor="main-drawer"
          className="fixed inset-0 z-40 bg-black/50 transition-opacity opacity-0 pointer-events-none peer-checked:opacity-100 peer-checked:pointer-events-auto lg:hidden"
        />

        {/* Main Content */}
        <main className="flex-1 flex flex-col h-full bg-content2 overflow-hidden relative">
          {children}
        </main>
      </div>
    );
  },
);
export default MainMenu;

export type MainMenuEntries = {
  [category: string]: {
    [name: string]: MainMenuItemData;
  };
};

export type MainMenuItemData = {
  href: string;
  recursive?: boolean;
};

const MainMenuItems = observer(({ entries }: { entries: MainMenuEntries }) => {
  return (
    <>
      {Object.entries(entries).map(([category, items], index) => {
        return (
          <MainMenuCategory key={index} category={category} items={items} />
        );
      })}
    </>
  );
});

const MainMenuCategory = observer(
  ({
    category,
    items,
  }: {
    category: string;
    items: { [name: string]: MainMenuItemData };
  }) => {
    return (
      <div className="space-y-2">
        <h3 className="px-2 text-xs font-semibold text-foreground-500 uppercase tracking-wider">
          {category}
        </h3>
        <div className="space-y-1">
          {Object.entries(items).map(([name, data], index) => {
            return <MainMenuItem item={data} display={name} key={index} />;
          })}
        </div>
      </div>
    );
  },
);

const MainMenuItem = observer(
  ({ item, display }: { item: MainMenuItemData; display: string }) => {
    const router = useRouter();
    const pathname = item.href;
    const active =
      (item.recursive ?? true)
        ? router.pathname.startsWith(pathname)
        : router.pathname === pathname;

    return (
      <Link href={item.href} passHref legacyBehavior>
        <Button
          fullWidth
          variant={active ? "flat" : "light"}
          color={active ? "primary" : "default"}
          className={`justify-start ${active ? "font-semibold" : ""}`}
          as="a"
        >
          {display}
        </Button>
      </Link>
    );
  },
);

const MainMenuLogo = observer(() => {
  return (
    <div className="px-2">
      <img
        src="/energetik_logo.png"
        className="w-12 h-12 mb-2"
        alt="Energetik Sabine Petschl"
      />
      <div className="text-xs text-foreground-500 leading-tight">
        <p className="font-semibold text-foreground">
          Energetik Sabine Petschl
        </p>
        <p>Wohlfühlen für Mensch und Tier</p>
      </div>
    </div>
  );
});

const ThemePicker = observer(() => {
  const store = useStore();
  const currentTheme = store.settingsStore.theme || "system";

  return (
    <Dropdown>
      <DropdownTrigger>
        <Button isIconOnly variant="light" title="Change Theme">
          <Palette size={20} />
        </Button>
      </DropdownTrigger>
      <DropdownMenu
        aria-label="Theme selection"
        onAction={(key) =>
          store.settingsStore.setTheme(
            key === "system" ? null : (key as string),
          )
        }
        selectedKeys={[currentTheme]}
        selectionMode="single"
      >
        <DropdownItem key="system">System</DropdownItem>
        <DropdownItem key="light">Light</DropdownItem>
        <DropdownItem key="dark">Dark</DropdownItem>
      </DropdownMenu>
    </Dropdown>
  );
});

const LanguagePicker = observer(() => {
  const store = useStore();
  const currentLang = store.settingsStore.language || "en";

  const knownLanguages = [
    { id: "en", name: "English", flag: "🇺🇸" },
    { id: "de", name: "Deutsch", flag: "🇩🇪" },
  ];

  return (
    <Dropdown>
      <DropdownTrigger>
        <Button isIconOnly variant="light" title="Change Language">
          <Languages size={20} />
        </Button>
      </DropdownTrigger>
      <DropdownMenu
        aria-label="Language selection"
        onAction={(key) => store.settingsStore.setLanguage(key as string)}
        selectedKeys={[currentLang]}
        selectionMode="single"
      >
        {knownLanguages.map((lang) => (
          <DropdownItem key={lang.id} startContent={lang.flag}>
            {lang.name}
          </DropdownItem>
        ))}
      </DropdownMenu>
    </Dropdown>
  );
});
