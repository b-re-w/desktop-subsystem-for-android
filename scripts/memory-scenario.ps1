<#
.SYNOPSIS
Runs one repeatable memory scenario on this PC's Android and prints one comparison table.

.DESCRIPTION
DESKTOP USE: this script touches nothing on the desktop by itself. It never moves the mouse,
types, focuses, resizes or closes a window. It only opens and closes apps through the local API
(`dsa launch` and `dsa stop-app`), which open and close their windows the way the user's own
launch and close would. The one step it cannot do through the API is starting a 4K video in
YouTube: it asks for that and waits for Enter, unless -NoPrompt is given (then YouTube is
measured with whatever it shows).

Steps, each followed by one reading:
  1. boot idle        Android started (started here when it is stopped) and left idle for
                      -IdleMinutes, no app opened by the script.
  2. FLO open         FLO (-Music) opened and left for -AppMinutes.
  3. YouTube 4K       YouTube (-Video) opened, a 4K video playing, for -AppMinutes.
  4. YouTube closed   -AfterCloseMinutes after `dsa stop-app` of YouTube, FLO still open.
  5. all closed       -AfterCloseMinutes after `dsa stop-app` of FLO too.

Each reading is `dsa memory --json` (the guest's memory as Android counts it, and the host
processes DSA names: the VM, the window manager, the renderer, the app windows) and the working
set and private bytes of every DSA process this user may read. Run it from PowerShell 7 as the
user whose Android is measured; an elevated prompt also reads the service and the VM's own
process. What cannot be read is printed as empty, never guessed. -Json writes every reading to
a file as well.

.EXAMPLE
pwsh -File scripts/memory-scenario.ps1
pwsh -File scripts/memory-scenario.ps1 -NoPrompt -IdleMinutes 2 -Json memory-scenario.json
#>
[CmdletBinding()]
param(
    [string]$Dsa = 'dsa',
    [string]$Music = 'skplanet.musicmate',
    [string]$Video = 'com.google.android.youtube',
    [double]$IdleMinutes = 5,
    [double]$AppMinutes = 2,
    [double]$AfterCloseMinutes = 5,
    [switch]$NoPrompt,
    # Where every reading goes as JSON, besides the table.
    [string]$Json
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Invoke-Dsa {
    $output = & $Dsa @args 2>&1
    if ($LASTEXITCODE -ne 0) { throw "dsa $($args -join ' ') failed: $output" }
    ($output | Where-Object { $_ -is [string] }) -join "`n"
}

function Get-DsaJson { Invoke-Dsa @args --json | ConvertFrom-Json }

function Mib($Kb) { if ($null -eq $Kb) { $null } else { [math]::Round([double]$Kb / 1024, 1) } }

function Wait-Minutes([double]$Minutes, [string]$Why) {
    if ($Minutes -le 0) { return }
    Write-Host ("  waiting {0} min: {1}" -f $Minutes, $Why)
    Start-Sleep -Seconds ([int]($Minutes * 60))
}

# The DSA processes on this PC this user may read: name, ID, working set and private bytes.
function Get-DsaProcesses {
    $apps = Join-Path $env:LOCALAPPDATA 'DSA\apps'
    Get-Process | Where-Object {
        $_.ProcessName -like 'dsa-*' -or $_.ProcessName -eq 'vmmem' -or
            $(try { $_.Path -and $_.Path -like (Join-Path $apps '*') } catch { $false })
    } | ForEach-Object {
        [pscustomobject][ordered]@{
            name = $_.ProcessName
            pid = $_.Id
            working_set_mib = Mib ($_.WorkingSet64 / 1KB)
            private_mib = Mib ($_.PrivateMemorySize64 / 1KB)
        }
    }
}

# One reading: what `dsa memory --json` says and what the host's process list says.
function Measure-Step([string]$Step) {
    Write-Host "  reading: $Step"
    $memory = try { Get-DsaJson memory } catch { Write-Warning "dsa memory failed: $_"; $null }
    $roles = [ordered]@{}
    if ($memory) {
        foreach ($p in @($memory.host)) {
            if (-not $roles.Contains($p.role)) { $roles[$p.role] = 0.0 }
            $roles[$p.role] += [double]$p.working_set_kb
        }
    }
    $byRole = [ordered]@{}
    foreach ($role in $roles.Keys) { $byRole[$role] = Mib $roles[$role] }
    $guest = if ($memory) { $memory.guest } else { $null }
    [ordered]@{
        step = $Step
        at = (Get-Date).ToString('s')
        state = $(try { (Get-DsaJson status).state } catch { $null })
        guest_total_mib = $(if ($guest) { Mib $guest.mem_total_kb } else { $null })
        guest_free_mib = $(if ($guest) { Mib $guest.mem_free_kb } else { $null })
        guest_available_mib = $(if ($guest) { Mib $guest.mem_available_kb } else { $null })
        guest_pss_mib = $(if ($guest) { Mib $guest.total_pss_kb } else { $null })
        guest_cached_mib = $(if ($guest) { Mib $guest.cached_kb } else { $null })
        zram_used_mib = $(if ($guest) { Mib $guest.zram_used_kb } else { $null })
        vm_beyond_guest_mib = $(if ($memory) { Mib $memory.vm_beyond_guest_kb } else { $null })
        host_total_mib = $(if ($memory) { Mib $memory.host_total_kb } else { $null })
        host_by_role_mib = $byRole
        processes = @(Get-DsaProcesses)
        memory = $memory
    }
}

$readings = [System.Collections.Generic.List[object]]::new()

Write-Host 'Memory scenario. This script opens and closes apps only through dsa launch and dsa stop-app; it touches nothing else on the desktop.'

# 1. Boot idle.
$state = (Get-DsaJson status).state
if ($state -eq 'Stopped') {
    Write-Host '  starting Android'
    Invoke-Dsa start --wait | Out-Null
}
Wait-Minutes $IdleMinutes 'Android idle'
$readings.Add((Measure-Step 'boot idle'))

# 2. FLO open.
Write-Host "  dsa launch $Music"
Invoke-Dsa launch $Music | Out-Null
Wait-Minutes $AppMinutes "$Music open"
$readings.Add((Measure-Step 'FLO open'))

# 3. YouTube with a 4K video.
Write-Host "  dsa launch $Video"
Invoke-Dsa launch $Video | Out-Null
if (-not $NoPrompt) {
    Read-Host "  start a 4K video in the YouTube window, then press Enter here" | Out-Null
}
Wait-Minutes $AppMinutes "$Video playing"
$readings.Add((Measure-Step 'YouTube 4K'))

# 4. YouTube closed.
Write-Host "  dsa stop-app $Video"
Invoke-Dsa stop-app $Video | Out-Null
Wait-Minutes $AfterCloseMinutes "after closing $Video"
$readings.Add((Measure-Step 'YouTube closed'))

# 5. All closed.
Write-Host "  dsa stop-app $Music"
Invoke-Dsa stop-app $Music | Out-Null
Wait-Minutes $AfterCloseMinutes 'after closing every app'
$readings.Add((Measure-Step 'all closed'))

# The comparison: one row per step, the host processes' working sets by role and the guest's
# memory, then each DSA process's working set by step.
$roleNames = @($readings | ForEach-Object { $_.host_by_role_mib.Keys } | Select-Object -Unique)
$rows = foreach ($r in $readings) {
    $row = [ordered]@{
        step = $r.step
        state = $r.state
        'host total' = $r.host_total_mib
    }
    foreach ($role in $roleNames) { $row[$role] = if ($r.host_by_role_mib.Contains($role)) { $r.host_by_role_mib[$role] } else { $null } }
    $row['VM beyond guest'] = $r.vm_beyond_guest_mib
    $row['guest PSS'] = $r.guest_pss_mib
    $row['guest free'] = $r.guest_free_mib
    $row['guest avail'] = $r.guest_available_mib
    $row['guest total'] = $r.guest_total_mib
    $row['zram'] = $r.zram_used_mib
    [pscustomobject]$row
}
Write-Host ''
Write-Host 'Memory by step (MiB; host columns are working sets by the role dsa memory gives):'
$rows | Format-Table -AutoSize | Out-String -Width 400 | Write-Host

$names = @($readings | ForEach-Object { $_.processes | ForEach-Object { $_.name } } | Select-Object -Unique)
$byProcess = foreach ($name in $names) {
    $row = [ordered]@{ process = $name }
    foreach ($r in $readings) {
        $sum = ($r.processes | Where-Object { $_.name -eq $name } | Measure-Object -Property working_set_mib -Sum).Sum
        $row[$r.step] = $(if ($null -ne $sum) { [math]::Round($sum, 1) } else { $null })
    }
    [pscustomobject]$row
}
Write-Host 'Working set of each DSA process by step (MiB):'
$byProcess | Format-Table -AutoSize | Out-String -Width 400 | Write-Host

if ($Json) {
    $readings | ConvertTo-Json -Depth 12 | Set-Content -Path $Json -Encoding utf8
    Write-Host "readings written to $Json"
}
