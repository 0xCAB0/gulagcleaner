import os
from pdfrw import PdfReader, PdfWriter
from pdfrw.findobjs import wrap_object , find_objects
from pdfrw.objects import PdfName
from pikepdf import Pdf

def deembed(pdf_path):
    '''
    Deembeds the pdf and creates a new PDF in the same folder with each embedded page
    in a new page.
    
    Args:
        pdf_path: The path where the pdf file is located.
        
    Returns:
        return_msg: Dict. with three values:
            Success: bool indicating whether the process was successful.
            return_path: If successful, returns the path of the deembedded file.
            Error: If unsuccessful, returns a description of the error.
    '''
    
    return_msg={"Success":False,"return_path":"","Error":""}
    try:
        if pdf_path[:-4]!=".pdf":
            return_msg["Success"]=False
            return_msg["Error"]="File is not a .pdf file."
            return return_msg
        prepdf=Pdf.open(pdf_path)
        prepdf.save(pdf_path[:-4]+"_inter.pdf")
        prepdf.close()

        pdf = PdfReader(pdf_path[:-4]+"_inter.pdf")
        xobjs=list(find_objects(pdf.pages,valid_subtypes=(PdfName.Form, PdfName.Dummy)))
        páginas=[]
        for item in xobjs:
            páginas.append(wrap_object(item, 1000, 0.5*72))
        if len(xobjs)==0:
            os.remove(pdf_path[:-4]+"_inter.pdf")
            return_msg["Success"]=False
            return_msg["Error"]="No embedded pages found."
            return return_msg

        output=pdf_path[:-4]+"_deembedded.pdf"
        writer = PdfWriter(output)
        writer.addpages(páginas)
        writer.write()

        os.remove(pdf_path[:-4]+"_inter.pdf")
        return_msg["Success"]=True
        return_msg["return_path"]=output
        return return_msg
    except:
        return_msg["Success"]=False
        return_msg["Error"]="Unknown internal error."
        return return_msg


if __name__ == "__main__":
    print('Call from the "pdfu" command.')
    