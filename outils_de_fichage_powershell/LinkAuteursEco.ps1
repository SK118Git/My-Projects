$fileAutEco = Get-ChildItem 'C:\Users\sasha\OneDrive\Documents\ObWG\ObWgit\Auteurs\Eco\'
$fileCours = Get-ChildItem 'C:\Users\sasha\OneDrive\Documents\ObWG\ObWgit\EC1\ESH\'
$bannedAut = 'sen', 'cette', 'kahn', 'nec', 'nek', 'neg', 'list', 'say', 'autor', 'hos', 'mill', 'amin', 'berger', 'ralle', 'sell', 'allemand', 'allan', 'blanc', 'bell', 'ford', 'march', 'aron', 'blanc', 'marchand', 'long', 'bureau', 'cole'
$bannedAutMaj = 'Cette', 'Allemand', 'Allan', 'Autor', 'Jean', 'Jones', 'Blanc', 'Bell', 'Simon', 'Engel', 'Mead'

#what about the !notes

foreach($_ in $fileCours){
    #$file1 = Get-Content $_.FullName
    #Write-Host $file1
    #Pause
    (Get-Content $_.FullName).replace('[', '') | Set-Content $_.FullName
    (Get-Content $_.FullName).replace(']', '') | Set-Content $_.FullName
    #$file = Get-Content $_.FullName
    Write-Host "Checking:" $_
    foreach ($f in $fileAutEco){
	Write-Host "Checking:" $f
        $filenameStr = ($f).toString()
        $withotxt = $filenameStr.replace('.md', '')
        $withottxtLC = $withotxt.ToLower()
        if (($withotxt -notin $bannedAutMaj)){
            (Get-Content $_.FullName).replace($withotxt, '[[' +$withotxt+ ']]') | Set-Content $_.FullName
        }
        if (($withottxtLC -notin $bannedAut)){
            (Get-Content $_.FullName).replace($withottxtLC, '[[' +$withottxtLC+ ']]') | Set-Content $_.FullName
        }
    }
}


