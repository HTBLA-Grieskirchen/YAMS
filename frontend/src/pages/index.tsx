import type {NextPage} from 'next'
import {observer} from "mobx-react";
import {useState} from "react";
import {query, useLive} from "../libs/database";
import Link from "next/link";
import notification from "../libs/notification";
import {useStore} from "../stores";
import dialog from "../libs/dialog";

const Home: NextPage = observer(() => {
    const [entryText, setEntryText] = useState("")
    const [entries, refreshEntries] = useLive("SELECT content FROM entry ORDER BY content")
    const entriesExtracted = entries.response?.length ?? -1 > 0 ? entries.response![0].result : []

    async function addEntry(text: string) {
        console.log("Adding entry")
        await query("CREATE entry SET content = $content", {
            "content": text
        })

        refreshEntries()
    }

    const store = useStore()

    return (
        <main className="">
            <div>
                <div className="flex flex-row space-x-4">
                    <div className="flex flex-col space-y-4">
                        <button onClick={e => notification.info({message: "Has been clicked", title: "routi"}, 10, {
                            "Remove": {
                                action: () => true,
                                disabled: () => store.dialogStore.dialogsShown()
                            },
                            "Remove Wait": {
                                action: async () => {
                                    function sleep(ms: number) {
                                        return new Promise(resolve => {
                                            setTimeout(resolve, ms)
                                        });
                                    }

                                    await sleep(2500)
                                    return true
                                }
                            },
                            "Alert": {
                                action: () => {
                                    alert("This is an alert")
                                    return false
                                }
                            }
                        })} className="text-lg">
                            Notify!
                        </button>
                        <button
                            onClick={e => dialog((close) => <div className="flex flex-row space-x-2">
                                {store.notificationStore.currentNotifications().length}
                                <button onClick={e => dialog((close) => <div className="flex flex-row space-x-2">
                                    {store.notificationStore.currentNotifications().length}
                                    <button onClick={e => close()}>Click to close all</button>
                                    <button onClick={e => close(false)}>Click to close this</button>
                                </div>)}>New Dialog
                                </button>
                                <button onClick={e => close()}>Click to close</button>
                            </div>)}
                            className="text-lg">
                            Dialog!
                        </button>
                    </div>
                    <Link href="/addresses/country">To Lands</Link>
                    <div className="flex flex-col space-y-2">
                        <input onChange={(e) => setEntryText(e.target.value)}/>
                        <button className="border rounded-lg bg-gray-400 text-black hover:bg-gray-200"
                                onClick={(e) => addEntry(entryText)}>Add the Entry
                        </button>
                    </div>
                    <div className="flex flex-col">
                        {entriesExtracted.map((item: any, index: number) => <p
                            key={index}>{item.content}</p>)}
                    </div>
                </div>
            </div>
        </main>
    )
})

export default Home
