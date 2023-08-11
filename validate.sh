#! /usr/bin/env bash
err=0

#for f in `find . -name "*.xml"`
for f in `find . -wholename "*eds/SEDS**.xml"` `find . -wholename "*eds/test**.xml"`
do
    xmllint --noout --schema eds/schema/seds.xsd $f
    if (($? != 0)); then
        err=$((err + 1))
    fi
done

if ((${err} > 0)); then
    echo "There were errors while parsing"
    exit 1
fi
