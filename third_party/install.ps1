Invoke-WebRequest https://download.steinberg.net/sdk_downloads/asiosdk_2.3.3_2019-06-14.zip -OutFile asiosdk_2.3.3_2019-06-14.zip

Expand-Archive -Path asiosdk_2.3.3_2019-06-14.zip -DestinationPath .

Rename-Item -Path asiosdk_2.3.3_2019-06-14 -NewName asiosdk

Remove-Item -Path asiosdk_2.3.3_2019-06-14.zip
