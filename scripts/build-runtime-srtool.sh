# Docker command to generate JSON blob of the runtime
CMD="docker run \
  -i \
  --rm \
  -e CARGO_NET_GIT_FETCH_WITH_CLI=true \
  -e PACKAGE=moonbase-runtime \
  -e RUNTIME_DIR=runtime/moonbase \
  -v ${PWD}:/build \
  -v /home/${USER}/srtool/.ssh:/home/builder/.ssh \
  -v /home/${USER}/srtool/entrypoint.sh:/srtool/entrypoint.sh \
  --entrypoint /srtool/entrypoint.sh \
  purestake/srtool:1.62.0 \
    build --app --json -cM"

# Here we run the command and stream the output (JSON blob) to a variable
stdbuf -oL $CMD | {
  while IFS= read -r line
  do
      echo ║ $line
      JSON="$line"
  done

  echo ::set-output name=json::$JSON

  PROP=`echo $JSON | jq -r .runtimes.compact.prop`
  echo ::set-output name=proposal_hash::$PROP

  WASM=`echo $JSON | jq -r .runtimes.compact.wasm`
  echo ::set-output name=wasm::$WASM

  Z_WASM=`echo $JSON | jq -r .runtimes.compressed.wasm`
  echo ::set-output name=wasm_compressed::$Z_WASM

  IPFS=`echo $JSON | jq -r .runtimes.compact.ipfs`
  echo ::set-output name=ipfs::$IPFS
}
