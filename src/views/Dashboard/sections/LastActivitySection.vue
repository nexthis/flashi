<template>
    <q-card flat>
        <div v-if="isSuccess && isSuccessDevice">
            <q-list>
                <q-item-label header>Activity</q-item-label>
                <q-item v-for="item in data" :key="item.client">
                    <!-- <q-item-section avatar>
                        <q-icon color="positive" name="mdi-lan-connect" />
                    </q-item-section> -->

                    <q-item-section>
                        <q-item-label>{{ timeSince(item.createdAt.toDate()) }}</q-item-label>
                        <q-item-label caption>
                            {{ dataDevices?.find((device) => device.key === item.client)?.name }}
                            -> to ->
                            {{ dataDevices?.find((device) => device.key === item.server)?.name }}
                        </q-item-label>
                    </q-item-section>
                </q-item>
            </q-list>
        </div>
        <div v-if="isLoading && isLoadingDevice">
            <q-circular-progress indeterminate rounded size="50px" color="secondary" class="q-ma-md" />
        </div>
    </q-card>
</template>

<script setup lang="ts">
import { useActivityList } from "@/composables/queries/useActivityList"
import { useDeviceList } from "@/composables/queries/useDeviceList"
import { timeSince } from "@/utils/time"

const { data, isSuccess, isLoading } = useActivityList()
const { data: dataDevices, isSuccess: isSuccessDevice, isLoading: isLoadingDevice } = useDeviceList()
</script>
