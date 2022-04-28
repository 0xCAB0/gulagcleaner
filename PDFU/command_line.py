from PDFU import PDFU_extract
from os.path import exists

def main():
    '''
    Main function called from the "pdfu" CLI command.
    The pdfu command takes an argv for the path and tries to deembed the pages inside it.
    The pages are saved in a new PDF in the same folder.
    '''
    
    import sys
    if len(sys.argv)>1:
        arg = sys.argv[1]
        if exists(arg):
            return_msg=PDFU_extract.deembed(arg)
            
            if return_msg["Success"]:
                print("Deembedding successful. File saved in",return_msg["return_path"])
            else:
                print("Error:",return_msg["Error"])
        else:
            print("File not found.")
    else:
        print('Usage: pdfu "filename"')
    
if __name__ == "__main__":
    print('Call from the "pdfu" command.')