import { redirect } from "next/navigation";

import { paths } from "@/lib/navigation";

export default function HomePage() {
  redirect(paths.klienten);
}
