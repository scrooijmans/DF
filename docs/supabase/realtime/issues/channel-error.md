CHANNEL ERRORS - What are they?
#22153
@kerbo
kerbo
on Mar 21, 2024 · 1 comments · 2 replies
Return to top

kerbo
on Mar 21, 2024
I often get Channel Errors. There is no information on what they are and under what conditions you would get them.

In my case, I am using the Flutter API, Realtime database channels, running using the Ios simulator and connecting to the Supabase server (not running locally). The channel code looks like:

channel.onPostgresChanges (.....).subscribe(status, error) {...}

Whenever the status is equal to RealtimeSubscribeStatus.channelError, I log the event to my console. The error value is always NULL. Errors usually occur when the channel is open and the application is idle (like overnight).

My questions are:

What are Channel Errors?
Under what conditions would Supabase throw a Channel error?
How can you get more information on the actual error?
Is it ok to ignore these events?
Replies:1 comment · 2 replies

GaryAustin1
on Mar 21, 2024
Maintainer
Realtime can get errors for many reasons. Tabs going into background, network lost, power saving modes of mobile devices. Realtime will sometimes retry and reconnect. But if an event occurs during the reconnect process or while realtime is not connected it will be lost as there is no queue.

This repository and older discussion look at this.
https://github.com/GaryAustin1/Realtime2
https://github.com/orgs/supabase/discussions/5641

2 replies
@kerbo
kerbo
on Mar 21, 2024
Author
I concluded from looking at your links that if you get an error just reconnect the channel...that is if the status is NOT subscribed then unsubscribe (removeChannel) and subscribe again. The actual reason for the error is unimportant.

@GaryAustin1
GaryAustin1
on Mar 21, 2024
Maintainer
Yes. I don't try and figure out why. But I don't just reconnect as changes are missed. I refetch any missed data. I also just shut things down when errors occur and visibility is not set as waste of a connection to keep trying.

Preliminary! -- Basic code working with error restart, needs lots of cleanup and work for multiple subscriptons. The code is not the same as used to generate the trace at bottom. The test part is more automated now and just uses console.logs and a timed loop to send updates (inserts/deletes coming).

NOTE: This respository [supabase-live-table](https://github.com/openartmarket/supabase-live-table/blob/main/src/index.ts) by aslakhellesoy uses a slightly different approach to the problem and uses timestamps to insure integrity. It is also a more complete working solution versus my more test scenarios discussed below. The error handling/retries discussed here and my other Github discussion should work with that repository also as currently it does not implement retry on error and leaves that to the user.

# Realtime2

In a Supabase Github Discussion [How to obtain reliable realtime updates in the real world](https://github.com/orgs/supabase/discussions/5641)
I show issues with Realtime losing connection and propose a solution to keep a copy of data from the database updated reliably in the face of errors. The
biggest issue is not missing table changes while Realime is reconnecting AND not missing changes in the initial connection.

I proposed loading initial data after the subscription had succeeded in connecting. This is certainly better than loading the intial data before starting the subscription
process, but it turns out there is still a small window for missing changes.

This repository is about eliminating that hole and will show an example to deal with it in JS.

The diagrams below show the issues with loading initial data then subscribing and the issues with getting initial data after subscribing.
The 2nd diagram also hints at the solution of a queue to deal with the remaining gap.

![image](https://github.com/GaryAustin1/Realtime2/assets/54564956/539a9be0-628b-424d-a711-96ca0b8031bf)

In the 2nd case, all data changes are captured by realtime and sent to the client. The issue is that there is no initial data to update between getting subscribed and getting the inital data. By including a queue to capture these request as part of the payload event handler, the original update code from "reliable updates in the real world) can be used.

Example Captures (tests not in the current code):
These show the yellow event path.

1.

![image](https://github.com/GaryAustin1/Realtime2/assets/54564956/13c2c135-4b70-4293-9c12-eafd0729d6f2)

2. ![missing events](https://github.com/GaryAustin1/Realtime2/assets/54564956/7c7d7860-8cee-4bca-9e67-51aac0d56acc)

3.

![image](https://github.com/GaryAustin1/Realtime2/assets/54564956/cb42a131-2b15-400c-b9e1-ff05276bb547)

Note. Need to edit below as the two yellows are actually green events. Must have been late. Will update with yellow event.
![](https://github.com/GaryAustin1/Realtime2/blob/4f5a19444a90fd07ac3f74c66566ef18bc23f166/DataRuns.drawio.png)

Note: I purposely moved API update calls around and in the event handler to increase chances of finding all the different cases occuring. This testing method is really not the best, or easily documented, but the idea is to cause database updates to occur all around the subscription and initial database table load.

---

I've been looking at the reliability of Supabase realtime for awhile and conclude that just doing the standard examples of starting a subscription and letting realtime handle re-connections in the background will cause loss of data changes. When faced with a lost connection or heartbeat the realtime code attempts to reconnect and resume BUT loses any updates on server during this time.

You have to put an error monitor in .subscribe() to see these connection errors (the payload portion of the code will look fine because of the reconnect attempts realtime is doing):
.subscribe((status) => {console.log('status - ', status)})

After testing on several browsers on a Windows desktop as well as mobile devices with (Safari and Chrome) it becomes clear subscriptions are lost in all cases (except Firefox on Windows) when the tab running realtime is not in focus, or device goes to sleep. Of course loss of internet connection even briefly can cause the same issue. Basic realtime examples in Supabase docs will all look like they are running normally when tab returns.

I'm looking at the following flow to be able to use realtime reliably (never miss a subscribed database change).

image

At first I was going to just use visibility of the page, but then if you just tab away from a page for a minute, you have to reload all your monitored realtime data.

There are other approaches in addition to above using timestamps that might reduce amount of data needing to be refreshed.

Here are some examples of realtime in background/sleeping tabs:

Windows Edge (Chrome is similar) on background tab:
edge-windows background

iPad/Safari:
Note in this case the device goes to sleep and errors from earlier in time show up after tab comes back.
android

Android/Chrome (note results differ between powered and battery operation, but all still lose subscription in time):
image

Replies:8 comments · 22 replies

GaryAustin1
on Mar 4, 2022
Maintainer
Author
I've implemented the above flowchart and done limited testing using this code structure.
So far this just handles a table or subset of table with limit, but would work for a single row with a filter also.
The code will keep the data array store updated thru failures in internet, tabs in background and devices going into power saving mode.
It also will allow hidden tabs to stay alive (not need reload) as long as realtime does not error (which will occur after minutes in many cases).

    let dataArrayStore = []  //store table data here
    const idCol = "id"
    const table = "messages"
    let hiddenError = false

    function visibilityListener() {
        if (document.visibilityState === "visible" && hiddenError) {
            startStream()  //going visible, start stopped stream
        }
    }

    document.addEventListener("visibilitychange", visibilityListener);

    startStream()

    function startStream () {
        hiddenError = false
        const mySubscription = supabase
            .from(table)
            .on('*', payload => {
                realtimeUpdate(payload.eventType, payload)
            })
            .subscribe((status) => {
                if (status === "SUBSCRIPTION_ERROR") {
                    if (document.visibilityState === "hidden") {    // page visible so let realtime reconnect and reload data
                        supabase.removeSubscription(mySubscription)
                        hiddenError = true
                    }
                }
                if (status === "SUBSCRIBED") {
                    realtimeUpdate("RELOAD", {})
                }
            })
    }

    async function realtimeUpdate(eventType,payload) {
        switch (eventType)  {
            case "INSERT":
                dataArrayStore.unshift(payload.new)
                break
            case "UPDATE":
                let objIndex1 = dataArrayStore.findIndex((obj => obj[idCol] === payload.new[idCol]));
                if (objIndex1 !== -1)
                    dataArrayStore[objIndex1] = payload.new
                break
            case "DELETE":
                let objIndex2 = dataArrayStore.findIndex((obj => obj[idCol] === payload.old[idCol]));
                if (objIndex2 !== -1)
                    dataArrayStore.splice(objIndex2, 1)
                break
            case "RELOAD":
                const {data, error} = await supabase
                    .from(table)
                    .select('*')
                    .order(idCol, {ascending: false})
                    .limit(5)
                dataArrayStore = data
        }
    }

This yields the following when run in Chrome tab that errors after 5 minutes hidden.

image
