# Registers a Vulkan driver manifest under HKLM\SOFTWARE\Khronos\Vulkan\Drivers (enabled).
# The runner's processes run elevated, and the Vulkan loader then ignores VK_DRIVER_FILES and
# VK_ICD_FILENAMES ("Loader is running with elevated permissions"); it lists only the drivers
# in the registry. The jobs still set both variables: DSA reads them to take the named drivers
# as they are, once it has checked they are registered here.
param([Parameter(Mandatory = $true)][string]$Manifest)
$ErrorActionPreference = 'Stop'
$key = "HKLM:\SOFTWARE\Khronos\Vulkan\Drivers"
New-Item -Path $key -Force | Out-Null
New-ItemProperty -Path $key -Name $Manifest -Value 0 -PropertyType DWord -Force | Out-Null
Write-Host "Registered Vulkan driver $Manifest"
Get-Item $key | Select-Object -ExpandProperty Property | ForEach-Object { Write-Host "  $_" }
