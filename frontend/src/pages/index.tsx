import type {NextPage} from 'next'
import {observer} from "mobx-react";
import {useState} from "react";
import {query, useQuery} from "../libs/dbConnection";

const Home: NextPage = observer(() => {
    const [entryText, setEntryText] = useState("")
    const [entries, refreshEntries] = useQuery("SELECT content FROM entry ORDER BY content", undefined, 1000)
    const entriesExtracted = entries.length > 0 ? entries[0].result : []

    async function addEntry(text: string) {
        console.log("Adding entry")
        await query("CREATE entry SET content = $content", {
            "content": text
        })

        refreshEntries()
    }

    return (
        <main className="">
            <div>
                <div className="flex flex-row space-x-4">
                    <p className="text-lg">Hello World!</p>
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
