#!/bin/bash -eux

EFM_VERSION=${1:-4.5}
EFM_CLUSTER_NAME=${2:-main}

# If EFM's PID file does not exist, this node shouldn't be part of the cluster
if [ ! -f /var/run/efm-${EFM_VERSION}/${EFM_CLUSTER_NAME}.pid ]; then
	exit 1;
fi
# Test if EFM is still running
if [ ! ps -p $(cat /var/run/efm-${EFM_VERSION}/${EFM_CLUSTER_NAME}.pid) -o pid= > /dev/null ]; then
	exit 1;
fi

if [ -f /var/run/efm-${EFM_VERSION}/efm_trigger_p ]; then
	exit 0;
fi
exit 1
