import java.util.ArrayList;
import java.util.concurrent.ThreadLocalRandom;


public class AI {
    static Procs CurGameState;
    int[][] CurBoard;
    int PlayerTagAI;

    public AI(Procs GameState, int PlayerTag){
        CurGameState = GameState;
        CurBoard = CurGameState.Board;
        PlayerTagAI = PlayerTag;
    }

    public int[] Where2(int k){
        int x = 0;
        int y = 0;
        switch(k){
            case 0:
                x = 0;
                //lin0cols
                for (int j = 0; j<3; j++){
                    if (CurBoard[x][j] == 0){
                        y = j;
                    }
                }
                break;
            case 1:
                x = 1;
                //lin1cols
                for (int j = 0; j<3; j++){
                    if (CurBoard[x][j] == 0){
                        y = j;
                    }
                }
                break;
            case 2:
                x = 2;
                for (int j = 0; j<3; j++){
                    if (CurBoard[x][j] == 0){
                        y = j;
                    }
                }
                //lin2cols
                break;
            case 3:
                y = 0;
                for (int i = 0; i<3; i++){
                    if (CurBoard[i][y] == 0){
                        x = i;
                    }
                }
                //linscol0
                break;
            case 4:
                y = 1;
                //linscol1
                for (int i = 0; i<3; i++){
                    if (CurBoard[i][y] == 0){
                        x = i;
                    }
                }
            
                break;
            case 5:
                y = 2;
                for (int i = 0; i<3; i++){
                    if (CurBoard[i][y] == 0){
                        x = i;
                    }
                }
                //linscol2
                break;
            case 6:
                for (int i = 0; i<3; i++){
                    for (int j = 0; j<3; j++){
                        if (CurBoard[i][j] == 0){
                            x = i;
                            y = j;
                        }
                    }
                }
                //Diag
                break;
            case 7:
            int i = 2;
                while (i >= 0){
                    for (int j = 0; j<3; j++){
                        if (CurBoard[i][j] == 0){
                            x = i;
                            y = j;
                        }
                    }
                    i = i - 1;
                }
                //Adiag
                break;
            }
        int[] Move = {x, y};
        return Move;
    }

    public ArrayList<Integer> Adj(int k, int lenArr){
        ArrayList<Integer> AdjPlaces = new ArrayList<Integer>();
        if (k-1 >= 0 ){
            AdjPlaces.add(k-1);
        }
        if (k+1 <= 2){
            AdjPlaces.add(k+1);
        }
        return AdjPlaces;
    }


    public int[] AdjacencyBonus(){
        int XConsider, YConsider;
        ArrayList<Integer> PosMoves = CurGameState.PossibleMoves();
        int indexS = 0;
        int[] sumVals = new int[PosMoves.size()/2];
        for (int h=0; h<PosMoves.size(); h+=2){
            XConsider = PosMoves.get(h);
            YConsider = PosMoves.get(h+1);
            //System.out.println(XConsider + "and" + YConsider);
            ArrayList<Integer> AdjPlacesX = Adj(XConsider, 3);
            ArrayList<Integer> AdjPLacesY = Adj(YConsider, 3);
        
            for (int i: AdjPlacesX){
                //System.out.println("Checking"+ i + " and " + YConsider);
                sumVals[indexS] = sumVals[indexS] +  CurBoard[i][YConsider];
            }
            for (int j:AdjPLacesY){
                //System.out.println("Checking"+ XConsider + " and " + j);
                sumVals[indexS] = sumVals[indexS] + CurBoard[XConsider][j];
            }

            for (int i: AdjPlacesX){
                for (int j:AdjPLacesY){
                //System.out.println("Checking"+ i + " and " + j);
                sumVals[indexS] = sumVals[indexS] +  CurBoard[i][j];
                }
            }

            //System.out.println("got out of loop index" + indexS);
            indexS += 1;
        }
        return sumVals;
    }    

    public int[] CheckBestMove(){
        int x;
        int y; 
        int[] AdjencyVals = AdjacencyBonus();
        int ReqIndex = 0;
        int[] DoubleCheck = CurGameState.ArraySums();
        //for (int k:AdjencyVals){
            //System.out.println(k);
        //}
        if (Procs.ArrContains(DoubleCheck, PlayerTagAI* 2)){
            int WinMethod = Procs.IndexNArray(DoubleCheck, PlayerTagAI*2);
            x = Where2(WinMethod)[0];
            y = Where2(WinMethod)[1];
        }
        else if (Procs.ArrContains(DoubleCheck, -PlayerTagAI*2)){
            int WinMethod = Procs.IndexNArray(DoubleCheck, -PlayerTagAI*2);
            x = Where2(WinMethod)[0];
            y = Where2(WinMethod)[1];
        }
        else if (Procs.ArrContains(AdjencyVals, PlayerTagAI*2) == true){
            ReqIndex = Procs.IndexNArray(AdjencyVals, PlayerTagAI*2);
            x = CurGameState.PossibleMoves().get(2*ReqIndex);
            y = CurGameState.PossibleMoves().get(2*ReqIndex+1);
            //System.out.println("2Pt case");        
        }
        else if (Procs.ArrContains(AdjencyVals, -PlayerTagAI*2) == true){
            ReqIndex = Procs.IndexNArray(AdjencyVals, -PlayerTagAI*2);
            x = CurGameState.PossibleMoves().get(2*ReqIndex);
            y = CurGameState.PossibleMoves().get(2*ReqIndex+1);
            //System.out.println("-2Pt case");
        }
        else if (Procs.ArrContains(AdjencyVals, PlayerTagAI*1) == true){
            ReqIndex = Procs.IndexNArray(AdjencyVals, PlayerTagAI*1);
            x = CurGameState.PossibleMoves().get(2*ReqIndex);
            y = CurGameState.PossibleMoves().get(2*ReqIndex+1);
            //System.out.println("Pt case");
        }
        else if (Procs.ArrContains(AdjencyVals, -PlayerTagAI*1) == true){
            ReqIndex = Procs.IndexNArray(AdjencyVals, -PlayerTagAI*1);
            int x_0 = CurGameState.PossibleMoves().get(2*ReqIndex);
            int y_0 = CurGameState.PossibleMoves().get(2*ReqIndex+1);
            int randomX = x_0;
            int randomY = y_0;
            while ((x_0 == randomX) &&  (y_0 == randomY)){
                // nextInt is normally exclusive of the top value,
                // so add 1 to make it inclusive
                int max = (int)((CurGameState.PossibleMoves().size()+1)/2);
                randomX = 2*ThreadLocalRandom.current().nextInt(0, max);
                randomY = 2*ThreadLocalRandom.current().nextInt(0, max)+1;
            }
            x = CurGameState.PossibleMoves().get(randomX);
            y = CurGameState.PossibleMoves().get(randomY);
            //System.out.println("-Pt case");
        }
        else{
            x = CurGameState.PossibleMoves().get(0);
            y = CurGameState.PossibleMoves().get(1);
            //System.out.println("none case");
        }
        //System.out.println("actually " + x + "and " + y);
        int[] Move = {x, y};
        return Move;
    }
}
