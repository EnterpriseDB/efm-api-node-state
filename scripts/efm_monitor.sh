#!/bin/bash -eux
# Script in charge of creating the trigger file if they don't
# exist.

EFM_VERSION=${1:-"4.4"}

# Checking if the trigger files exist, if they exist, then no
# need for further verification.
if [ -f /var/run/efm-${EFM_VERSION}/efm_trigger_p ]; then
  exit 0
fi
if [ -f /var/run/efm-${EFM_VERSION}/efm_trigger_s ]; then
  exit 0
fi

# When the trigger file does not exist, we will create it, based
# on the return of the efm node-status-json command.
NODE_STATUS=$(/usr/edb/efm-${EFM_VERSION}/bin/efm node-status-json main)
NODE_TYPE=$(echo $NODE_STATUS | sed -E "s/.*type\":\"([^\"]+)\".*/\1/")
if [ "${NODE_TYPE}" == "Primary" ]; then
  /bin/touch /var/run/efm-${EFM_VERSION}/efm_trigger_p
fi
if [ "${NODE_TYPE}" == "Standby" ]; then
  /bin/touch /var/run/efm-${EFM_VERSION}/efm_trigger_s
fi
