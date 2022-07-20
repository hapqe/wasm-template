<script lang="ts">
  import Canvas from "./lib/Canvas.svelte";
  import * as Comlink from "comlink";

  import { threads } from "wasm-feature-detect";

  // @ts-ignore
  import * as Paralell from "../pkg-parallel";
  import * as SingleThreaded from "../pkg";

  (async () => {
    const useMt =
      (await threads()) && !(navigator.userAgent.indexOf("Firefox") > 0);
    let wasm = (
      useMt
        ? await Comlink.wrap(
            new Worker(new URL("./wasm-worker.js", import.meta.url), {
              type: "module",
            })
            // @ts-ignore
          ).handler
        : await SingleThreaded.default()
    ) as typeof Paralell;
    let one = wasm.with_no_args();
    console.log(await one);
  })();
</script>

<Canvas />
