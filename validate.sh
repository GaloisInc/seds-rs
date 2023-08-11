#! /usr/bin/env bash
for f in `find . -name "*.xml"`
do
    xmllint --noout --schema eds/schema/seds.xsd $f
    echo $?
done
