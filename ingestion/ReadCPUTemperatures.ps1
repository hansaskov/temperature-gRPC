(Get-WmiObject -Namespace "root/LibreHardwareMonitor" -Class "Sensor" | Where-Object {$_.SensorType -eq "Temperature" -and $_.Name -like "*Package*"} | Select-Object -First 1).Value

#(Get-WmiObject -Namespace "root/OpenHardwareMonitor" -Class "Sensor" | Where-Object {$_.SensorType -eq "Load" -and $_.Name -like "*Total*"} | Select-Object -First 1).Value
