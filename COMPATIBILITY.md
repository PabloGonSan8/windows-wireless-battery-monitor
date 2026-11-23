# Device compatibility

Community-reported results. A device "works" when its battery level shows in the
overlay. Missing yours? **[Report it](https://github.com/PabloGonSan8/windows-wireless-battery-monitor/issues/new?template=device_report.yml)**
— reports of devices that *don't* work are just as useful.

Rule of thumb: if *Windows Settings → Bluetooth & devices* shows a percentage for the
device, the app should show it too. If Settings doesn't, the app can't either.

| Device | Type | Works? | Windows | Notes | Report |
| --- | --- | :---: | --- | --- | --- |
| HUAWEI FreeBuds SE 3 | Earbuds | ✅ | 11 (build 26300) | | maintainer |

### Known not to work

| Device | Reason |
| --- | --- |
| Apple AirPods | Do not report battery level to Windows |
| Devices on a Logitech Unifying / Bolt USB receiver | Not Bluetooth from Windows' point of view (vendor protocol over USB) |

<sub>Maintainers: add one row per device report, link the issue in the last column, then close the issue.</sub>
