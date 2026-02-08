# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0
INSTANCE_ID=${1:-myso}
command=(
  cbt
  -instance
  "$INSTANCE_ID"
)
if [[ -n $BIGTABLE_EMULATOR_HOST ]]; then
  command+=(-project emulator)
fi

for table in objects transactions checkpoints checkpoints_by_digest watermark_alt epochs object_types; do
  (
    set -x
    "${command[@]}" createtable $table
    "${command[@]}" createfamily $table myso
    "${command[@]}" setgcpolicy $table myso maxversions=1
  )
done
"${command[@]}" setgcpolicy watermark myso maxage=2d
