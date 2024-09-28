$AuthListFolder = Get-ChildItem 'C:\Users\sasha\OneDrive\Desktop\Testingpsh\ListAuteursList'

foreach($_ in $AuthListFolder){
    Get-Content $_.FullName | ForEach-Object -Begin {$i = 1} -Process {
        $elemnt=  ($_ -split ' ')[0]
        if (-not($elemnt -match ".*\d+.") -and(-not($elemnt -cmatch "^[a-z]*$"))){ 
            if ($element -match ","){
                $element.Replace(",", " ")
            }
            if(-not($elemnt -cmatch "[^a-zA-Z0-9]+")){
                #Write-Host $elemnt
                $Newfile = "C:\Users\sasha\OneDrive\Documents\ObWG\ObWgit\Auteurs\Eco\" + $elemnt + ".md"
                #$Newfile2 = "C:\Users\sasha\OneDrive\Desktop\Testingpsh\Auteurs\" + $elemnt + ".md" 
                # Create the folder is the path doesn't exist   
                If (-not(test-path -Path $Newfile)) {
                    New-Item -Path $Newfile -ItemType File
                    #Write-Host $Newfile
                }
            }
        }
        $i += 1
    }
}



