$fileCours = Get-ChildItem 'C:\Users\sasha\OneDrive\Documents\ObWG\ObWgit\EC1\ESH\'
$bannedAut = 'sen', 'cette', 'kahn', 'nec', 'nek', 'neg', 'list', 'say', 'autor', 'hos', 'mill', 'amin', 'berger', 'ralle', 'sell', 'allemand', 'allan', 'blanc', 'bell', 'ford', 'march', 'aron', 'blanc', 'marchand', 'long'
$bannedAutMaj = 'Cette', 'Allemand', 'Allan', 'Autor', 'Jean', 'Jones', 'Blanc', 'Bell', 'Simon', 'Engel'
$bannedAutTot = $bannedAutMaj + $bannedAut

foreach($_ in $fileCours){
    foreach ($f in $bannedAutTot){
        #$filenameStr = ($f).toString()
        #$withotxt = $filenameStr.replace('.md', '')
        #$withottxtLC = $withotxt.ToLower()
        #if ($withotxt -notin $bannedAutMaj){

        #(Get-Content $_.FullName).replace('[[' +$f+ ']]', $f) | Set-Content $_.FullName
        #if ($withottxtLC -notin $bannedAut){
        #}
    }
    #(Get-Content $_.FullName).replace('[[[[', '') | Set-Content $_.FullName
    #(Get-Content $_.FullName).replace(']]]]', '') | Set-Content $_.FullName
}


