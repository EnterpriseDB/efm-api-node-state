#!/bin/bash -eux

EFM_VERSION=${1:-4.5}
EFM_CLUSTER_NAME=${2:-main}

IS_PRIMARY=$(/usr/edb/efm-${EFM_VERSION}/bin/efm node-status-json ${EFM_CLUSTER_NAME} | grep '"type":"Primary"' | grep -c '"db":"UP"');
if [ $IS_PRIMARY -eq "1" ];
then
	exit 0;
fi;
exit 1;
