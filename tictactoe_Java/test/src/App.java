import java.util.Scanner;

public class App {
    public static void main(String[] args) throws Exception {
        
        Procs GameState = new Procs();
        System.out.println("Which player do you want to pick, 1 for X and -1 for O");
        
        Scanner MainScanDude = new Scanner(System.in);
        int NewPlayerTag = MainScanDude.nextInt();

        AI ai = new AI(GameState, -NewPlayerTag);

        
        while (GameState.GameOver() == false) {
        if (NewPlayerTag == -1){
            int aimoveX = ai.CheckBestMove()[0];
            int aimoveY = ai.CheckBestMove()[1];
            System.out.println("AI move: X=" + aimoveX + " and  Y= " + aimoveY);
            GameState.PlayMoveP1(aimoveX, aimoveY);       
        }

        else {
            int XP1 = GameState.AskMoveX();
            int YP1 = GameState.AskMoveY();
            GameState.PlayMoveP1(XP1, YP1);
        }

        //GameState.ShowBoardRaw();
        GameState.ShowBoardNice();

        if (GameState.GameOver() == true){
            break;
        }

        if (NewPlayerTag == 1){
            int aimoveX = ai.CheckBestMove()[0];
            int aimoveY = ai.CheckBestMove()[1];
            System.out.println("AI move: X=" + aimoveX + " and  Y= " + aimoveY);
            GameState.PlayMoveP2(aimoveX, aimoveY);    
        }

        else{
            int XP2 = GameState.AskMoveX();
            int YP2 = GameState.AskMoveY();
            GameState.PlayMoveP2(XP2, YP2);
        }

        //GameState.ShowBoardRaw();
        GameState.ShowBoardNice();

        //System.out.println(GameState.MegaTrace());
        }

        GameState.WhoWin();
    }
}
