import java.util.ArrayList;
import java.util.Scanner;
// Create a Main class
public class Procs {
  int Board[][] = {{0,0,0}, {0,0,0}, {0,0,0}};  // Create a class attribute

  // Create a class constructor for the Main class
  public Procs() {
     // Set the initial value for the class attribute x
  }

  public static boolean ArrContains(int[] arr, int TargetValue){
    for (int s: arr){
        if (s == TargetValue){
            return true; 
        }
    }
    return false;
   }

   public static int IndexNArray(int[] arr, int TargetValue){
    int len = arr.length;
    for (int k = 0; k < len; k++){
        if (arr[k] == TargetValue){
            return k;
        }
    }
    return 0;
   }



  public void ShowBoardRaw(){
    for (int i=0; i<3; i++){
        for (int j=0; j<3; j++){
            System.out.print("|" + Board[i][j] + "|");
        };
        System.out.println();
    }
  }
  public void ShowBoardNice(){
    for (int i=0; i<3; i++){
        for (int j=0; j<3; j++){
          if (Board[i][j] == 1){
            System.out.print("|" + "X" + "|");
          }
          else if (Board[i][j] == -1){
            System.out.print("|" + "O" + "|");
          }
          else if (Board[i][j] == 0){
            System.out.print("|" + " " + "|");
          }
          
        };
        System.out.println();
    }
  }

  public int AskMoveX(){
    Scanner NewScanDude = new Scanner(System.in);
    System.out.println("Player make your X move: ");
    int P1MoveX = NewScanDude.nextInt();
    return P1MoveX; 
  }

  public int AskMoveY(){
    Scanner NewScanDude = new Scanner(System.in);
    System.out.println("Player make your Y move: ");
    int P1MoveY = NewScanDude.nextInt();
    return P1MoveY;
  }

  public void PlayMoveP1(int a, int b){
    Board[a][b] = 1;
  }
  public void PlayMoveP2(int a, int b){
    Board[a][b] = -1;
  }

  // what follows is pure retardation

  public int SumLin0Cols(){
    return Board[0][0] + Board[0][1] + Board[0][2];
  }


  public int SumLin1Cols(){
    return Board[1][0] + Board[1][1] + Board[1][2];
  }

  public int SumLin2Cols(){
    return Board[2][0] + Board[2][1] + Board[2][2];
  }
  
  public int SumLinsCol0(){
    return Board[0][0] + Board[1][0] + Board[2][0];
  }  
  
  public int SumLinsCol1(){
    return Board[0][1] + Board[1][1] + Board[2][1];
  }  

  public int SumLinsCol2(){
    return Board[0][2] + Board[1][2] + Board[2][2];
  }  

  public int SumDiag(){
    return Board[0][0] + Board[1][1] + Board[2][2];
  }

  public int SumAntiDiag(){
    return Board[0][2] + Board[1][1] + Board[2][0];
  }

  public int[] ArraySums(){
    int sumval1, sumval2, sumval3, sumval4, sumval5, sumval6, sumval7, sumval8;
    sumval1 = SumLin0Cols();
    sumval2 = SumLin1Cols();
    sumval3 = SumLin2Cols();
    sumval4 = SumLinsCol0();
    sumval5 = SumLinsCol1();
    sumval6 = SumLinsCol2();
    sumval7 = SumDiag();
    sumval8 = SumAntiDiag();
    int[] ArraySumsVal = {sumval1, sumval2, sumval3, sumval4, sumval5, sumval6, sumval7, sumval8};
    return ArraySumsVal;
  }

  // end of retardation

  public int MegaTrace(){
    int sumval = 0;

    int[] Arrsum = ArraySums();

    if ((ArrContains(Arrsum, 3))){
        sumval = 3 ;
    } else if (ArrContains(Arrsum, -3)){
        sumval = -3;
    }
    return sumval;
  }

  public ArrayList<Integer> PossibleMoves(){
    ArrayList<Integer> ReturnVal = new ArrayList<Integer>();
    for (int i=0; i<3; i++){
      for (int j = 0; j<3; j++){
        if (Board[i][j] == 0){
          ReturnVal.add(i);
          ReturnVal.add(j);
        }
      }
    }
    return ReturnVal;
  }

  public boolean GameOver(){
    int sumval = MegaTrace();
    if ((sumval == -3) || (sumval == 3)) {
        return true;
    }
    else if (PossibleMoves().size() == 0){
        return true;
    }
    else{
        return false;
    }
  }

  public void WhoWin(){
    int sumval = MegaTrace();
    if (sumval == 3){
        System.out.println("P1 wins!"); 
    }
    else if (sumval == -3){
        System.out.println("P2 wins!");
    }
    else if (PossibleMoves().size() == 0){
      System.out.println("It's a draw!");
    }

  }

}
