#!/bin/bash -eux

source $HOME/.cargo/env

VERSION="0.2.0"

cd /workspace

yum-builddep -y /workspace/packaging/rpm/efm-api-node-state.spec

rpmbuild \
	--clean \
	--define "pkgversion ${VERSION}" \
	--define "_topdir ${PWD}/tmp/rpm" \
	--define "_sourcedir ${PWD}" \
	-bb /workspace/packaging/rpm/efm-api-node-state.spec

mkdir -p ${PWD}/packaging/rpm/build/
cp ${PWD}/tmp/rpm/RPMS/*/*.rpm ${PWD}/packaging/rpm/build/.
chown ${1}:${2} -R ${PWD}/packaging/rpm/build/
ls -lha ${PWD}/packaging/rpm/build/*.rpm
