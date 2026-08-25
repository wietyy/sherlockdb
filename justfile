run:
    @tsc --outDir dist src/index.ts
    @node .

todo:
    @grep -Rni --exclude-dir=.git --exclude-dir=node_modules --exclude=justfile "TODO" .